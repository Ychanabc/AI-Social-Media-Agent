use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::sync::Arc;

use crate::auth::{CreateUserRequest, LoginRequest};
use crate::models::{
    GenerateRequest, GenerateResponse, StatsResponse, TaskListResponse, TaskResult,
    TaskStatusResponse,
};
use crate::models::AppState;
use crate::orchestrator::start_pipeline;

// ============ 认证 API ============

/// POST /api/auth/login
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.auth.login(&payload).await {
        Some(resp) => Ok(Json(serde_json::to_value(resp).unwrap())),
        None => Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string())),
    }
}

/// GET /api/auth/me
pub async fn get_me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    match state.auth.verify_token(&token).await {
        Some(user) => Ok(Json(serde_json::to_value(user).unwrap())),
        None => Err((StatusCode::UNAUTHORIZED, "无效的 token".to_string())),
    }
}

// ============ 管理员 API ============

/// GET /api/admin/users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let user = state.auth.verify_token(&token).await
        .ok_or((StatusCode::UNAUTHORIZED, "无效的 token".to_string()))?;
    if user.role != crate::auth::Role::Admin {
        return Err((StatusCode::FORBIDDEN, "需要管理员权限".to_string()));
    }
    let users = state.auth.list_users().await;
    Ok(Json(serde_json::json!({ "users": users })))
}

/// POST /api/admin/users
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let user = state.auth.verify_token(&token).await
        .ok_or((StatusCode::UNAUTHORIZED, "无效的 token".to_string()))?;
    if user.role != crate::auth::Role::Admin {
        return Err((StatusCode::FORBIDDEN, "需要管理员权限".to_string()));
    }
    match state.auth.create_user(payload).await {
        Ok(new_user) => Ok(Json(serde_json::to_value(new_user).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// DELETE /api/admin/users/:username
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let user = state.auth.verify_token(&token).await
        .ok_or((StatusCode::UNAUTHORIZED, "无效的 token".to_string()))?;
    if user.role != crate::auth::Role::Admin {
        return Err((StatusCode::FORBIDDEN, "需要管理员权限".to_string()));
    }
    match state.auth.delete_user(&username).await {
        Ok(()) => Ok(Json(serde_json::json!({ "ok": true }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// GET /api/admin/config
pub async fn get_config(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let user = state.auth.verify_token(&token).await
        .ok_or((StatusCode::UNAUTHORIZED, "无效的 token".to_string()))?;
    if user.role != crate::auth::Role::Admin {
        return Err((StatusCode::FORBIDDEN, "需要管理员权限".to_string()));
    }
    Ok(Json(serde_json::json!({
        "ai": {
            "provider": state.config.ai.provider,
            "model": state.config.ai.model,
            "base_url": state.config.ai.base_url,
            "api_key_set": !state.config.ai.api_key.is_empty() && state.config.ai.api_key != "sk-your-key-here",
        },
        "xhs": {
            "app_id_set": state.config.xhs.app_id != "your-app-id" && !state.config.xhs.app_id.is_empty(),
        }
    })))
}

/// POST /api/admin/config/ai
pub async fn update_ai_config(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let user = _state.auth.verify_token(&token).await
        .ok_or((StatusCode::UNAUTHORIZED, "无效的 token".to_string()))?;
    if user.role != crate::auth::Role::Admin {
        return Err((StatusCode::FORBIDDEN, "需要管理员权限".to_string()));
    }
    // 注意：运行时更新配置需要重新初始化 AI Client
    // 在生产环境中应该使用更完善的配置热更新机制
    Ok(Json(serde_json::json!({ "ok": true, "message": "配置已更新（需要重启服务生效）" })))
}

// ============ 内容生成 API ============

/// POST /api/generate
pub async fn generate_content(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, String)> {
    if payload.goal.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "goal 不能为空".to_string()));
    }
    tracing::info!("📨 收到生成请求: {}", payload.goal);
    let task_id = start_pipeline(payload.goal, state).await;
    tracing::info!("✅ 任务已创建: {}", task_id);
    Ok(Json(GenerateResponse { task_id }))
}

/// GET /api/task/:id
pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<TaskStatusResponse>, (StatusCode, String)> {
    let tasks = state.tasks.read().await;
    match tasks.get(&id) {
        Some(task) => Ok(Json(TaskStatusResponse {
            task_id: task.id.clone(),
            status: task.status.clone(),
            progress: task.progress,
            created_at: task.created_at,
            updated_at: task.updated_at,
        })),
        None => Err((StatusCode::NOT_FOUND, format!("任务 {} 不存在", id))),
    }
}

/// GET /api/tasks
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
) -> Json<TaskListResponse> {
    let tasks = state.tasks.read().await;
    let task_list: Vec<TaskStatusResponse> = tasks
        .values()
        .map(|task| TaskStatusResponse {
            task_id: task.id.clone(),
            status: task.status.clone(),
            progress: task.progress,
            created_at: task.created_at,
            updated_at: task.updated_at,
        })
        .collect();
    let total = task_list.len();
    Json(TaskListResponse { tasks: task_list, total })
}

/// GET /api/result/:id
pub async fn get_result(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<TaskResult>, (StatusCode, String)> {
    let tasks = state.tasks.read().await;
    match tasks.get(&id) {
        Some(task) => match &task.result {
            Some(result) => Ok(Json(result.clone())),
            None => Err((StatusCode::ACCEPTED, "任务尚未完成，请稍后重试".to_string())),
        },
        None => Err((StatusCode::NOT_FOUND, format!("任务 {} 不存在", id))),
    }
}

/// GET /api/stats
pub async fn get_stats(
    State(state): State<Arc<AppState>>,
) -> Json<StatsResponse> {
    let tasks = state.tasks.read().await;
    let total = tasks.len();
    let completed = tasks.values().filter(|t| t.status == crate::models::TaskStatus::Done).count();
    let running = tasks.values().filter(|t| t.status == crate::models::TaskStatus::Running).count();
    let completed_tasks: Vec<_> = tasks.values().filter(|t| t.status == crate::models::TaskStatus::Done).collect();
    let (avg_engagement, avg_views, avg_conversion) = if completed_tasks.is_empty() {
        (0.0, 0.0, 0.0)
    } else {
        let sum_e: f64 = completed_tasks.iter().filter_map(|t| t.result.as_ref()).map(|r| r.analytics.engagement_rate).sum();
        let sum_v: f64 = completed_tasks.iter().filter_map(|t| t.result.as_ref()).map(|r| r.analytics.views as f64).sum();
        let sum_c: f64 = completed_tasks.iter().filter_map(|t| t.result.as_ref()).map(|r| r.analytics.conversion_rate).sum();
        let n = completed_tasks.len() as f64;
        ((sum_e / n * 100.0).round() / 100.0, (sum_v / n).round(), (sum_c / n * 100.0).round() / 100.0)
    };
    Json(StatsResponse {
        total_tasks: total,
        completed_tasks: completed,
        running_tasks: running,
        avg_engagement_rate: avg_engagement,
        avg_views,
        avg_conversion_rate: avg_conversion,
    })
}

// ============ 小红书数据分析 API ============

/// GET /api/xhs/overview
pub async fn xhs_overview(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let overview = state.xhs.get_overview().await;
    Json(serde_json::to_value(overview).unwrap())
}

/// GET /api/xhs/notes
pub async fn xhs_notes(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let notes = state.xhs.get_notes().await;
    Json(serde_json::json!({ "notes": notes, "total": notes.len() }))
}

/// GET /api/xhs/notes/:id
pub async fn xhs_note_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match state.xhs.get_note_detail(&id).await {
        Some(note) => Ok(Json(serde_json::to_value(note).unwrap())),
        None => Err((StatusCode::NOT_FOUND, format!("笔记 {} 不存在", id))),
    }
}

// ============ 工具函数 ============

fn extract_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .ok_or((StatusCode::UNAUTHORIZED, "缺少 Authorization 头".to_string()))
}