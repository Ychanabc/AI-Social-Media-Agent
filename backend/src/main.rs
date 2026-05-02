mod api;
mod agents;
mod ai_client;
mod auth;
mod config;
mod models;
mod orchestrator;
mod xhs;

use axum::{routing::{get, post, delete}, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::models::AppState;
use crate::api::handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "ai_social_media_agent=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(AppState::new());

    tracing::info!("✅ 配置加载完成");
    tracing::info!("   AI Provider: {} | Model: {} | Configured: {}", 
        state.config.ai.provider, state.config.ai.model, state.config.ai_configured());
    tracing::info!("   XHS Configured: {}", state.config.xhs_configured());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // 认证 API
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::get_me))
        // 管理员 API
        .route("/api/admin/users", get(handlers::list_users))
        .route("/api/admin/users", post(handlers::create_user))
        .route("/api/admin/users/:username", delete(handlers::delete_user))
        .route("/api/admin/config", get(handlers::get_config))
        .route("/api/admin/config/ai", post(handlers::update_ai_config))
        // 内容生成 API
        .route("/api/generate", post(handlers::generate_content))
        .route("/api/task/:id", get(handlers::get_task))
        .route("/api/tasks", get(handlers::list_tasks))
        .route("/api/result/:id", get(handlers::get_result))
        .route("/api/stats", get(handlers::get_stats))
        // 小红书数据分析 API
        .route("/api/xhs/overview", get(handlers::xhs_overview))
        .route("/api/xhs/notes", get(handlers::xhs_notes))
        .route("/api/xhs/notes/:id", get(handlers::xhs_note_detail))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("🚀 AI Social Media Agent API 启动于 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}