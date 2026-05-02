use std::sync::Arc;
use uuid::Uuid;

use crate::agents::{AnalyticsAgent, ContentAgent, MediaAgent, PublishAgent, TopicAgent};
use crate::models::{AppState, Task, TaskResult, TaskStatus};

/// Orchestrator - 串联 Agent 执行的编排器
///
/// 执行流程:
/// 1. TopicAgent: 生成选题
/// 2. ContentAgent + MediaAgent: 并发执行生成内容和媒体
/// 3. PublishAgent: 安排发布
/// 4. AnalyticsAgent: 生成分析数据
pub struct Orchestrator;

impl Orchestrator {
    /// 异步执行完整的 Agent Pipeline
    pub async fn execute_pipeline(task_id: String, goal: String, state: Arc<AppState>) {
        tracing::info!("🚀 Orchestrator 开始执行任务 {} | 目标: {}", task_id, goal);

        // 更新状态为 running
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 10).await;

        // Step 1: Topic Agent - 生成选题
        let topics = match TopicAgent::execute(&goal).await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("TopicAgent 执行失败: {}", e);
                Self::update_task_status(&state, &task_id, TaskStatus::Failed, 0).await;
                return;
            }
        };
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 25).await;

        // Step 2: Content + Media 并发执行
        // ContentAgent 和 MediaAgent 可并发，Media 不依赖 Content 输出
        let topics_for_content = topics.clone();
        let (content_result, media_result) = tokio::join!(
            ContentAgent::execute(&topics_for_content),
            MediaAgent::execute_for_topics(&topics)
        );

        let contents = match content_result {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("ContentAgent 执行失败: {}", e);
                Self::update_task_status(&state, &task_id, TaskStatus::Failed, 0).await;
                return;
            }
        };
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 40).await;

        let media = match media_result {
            Ok(m) => m,
            Err(e) => {
                tracing::error!("MediaAgent 执行失败: {}", e);
                Self::update_task_status(&state, &task_id, TaskStatus::Failed, 0).await;
                return;
            }
        };
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 60).await;

        // Step 3: Publish Agent - 安排发布
        let publish = match PublishAgent::execute(&contents).await {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("PublishAgent 执行失败: {}", e);
                Self::update_task_status(&state, &task_id, TaskStatus::Failed, 0).await;
                return;
            }
        };
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 75).await;

        // Step 4: Analytics Agent - 生成分析数据
        let analytics = match AnalyticsAgent::execute(&goal).await {
            Ok(a) => a,
            Err(e) => {
                tracing::error!("AnalyticsAgent 执行失败: {}", e);
                Self::update_task_status(&state, &task_id, TaskStatus::Failed, 0).await;
                return;
            }
        };
        Self::update_task_status(&state, &task_id, TaskStatus::Running, 90).await;

        // 组装结果
        let result = TaskResult {
            task_id: task_id.clone(),
            goal: goal.clone(),
            topics,
            contents,
            media,
            publish,
            analytics,
        };

        // 写入结果并标记完成
        {
            let mut tasks = state.tasks.write().await;
            if let Some(task) = tasks.get_mut(&task_id) {
                task.result = Some(result);
                task.status = TaskStatus::Done;
                task.progress = 100;
                task.updated_at = chrono::Utc::now();
            }
        }

        tracing::info!("✅ Orchestrator 任务 {} 执行完成", task_id);
    }

    async fn update_task_status(
        state: &AppState,
        task_id: &str,
        status: TaskStatus,
        progress: u8,
    ) {
        let mut tasks = state.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = status;
            task.progress = progress;
            task.updated_at = chrono::Utc::now();
        }
    }
}

/// 创建新任务并启动 Pipeline
pub async fn start_pipeline(goal: String, state: Arc<AppState>) -> String {
    let task_id = Uuid::new_v4().to_string();
    let task = Task::new(task_id.clone(), goal.clone());

    // 写入任务
    {
        let mut tasks = state.tasks.write().await;
        tasks.insert(task_id.clone(), task);
    }

    // 启动异步 Pipeline
    let state_clone = state.clone();
    let tid = task_id.clone();
    tokio::spawn(async move {
        Orchestrator::execute_pipeline(tid, goal, state_clone).await;
    });

    task_id
}