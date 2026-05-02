use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Done,
    Failed,
}

/// 生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub goal: String,
}

/// 生成响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub task_id: String,
}

/// 任务状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusResponse {
    pub task_id: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskStatusResponse>,
    pub total: usize,
}

/// 选题结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub description: String,
    pub platform: String,
    pub hashtags: Vec<String>,
}

/// 内容版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentVersion {
    pub id: String,
    pub topic_id: String,
    pub title: String,
    pub body: String,
    pub tone: String,
    pub word_count: usize,
}

/// 媒体资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub content_id: String,
    pub url: String,
    pub media_type: String,
    pub width: u32,
    pub height: u32,
}

/// 发布信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishInfo {
    pub id: String,
    pub platform: String,
    pub scheduled_at: String,
    pub status: String,
}

/// 分析数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub views: u64,
    pub likes: u64,
    pub shares: u64,
    pub comments: u64,
    pub conversion_rate: f64,
    pub engagement_rate: f64,
    pub sentiment_score: f64,
}

/// 完整结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub goal: String,
    pub topics: Vec<Topic>,
    pub contents: Vec<ContentVersion>,
    pub media: Vec<MediaAsset>,
    pub publish: Vec<PublishInfo>,
    pub analytics: AnalyticsData,
}

/// 内部任务结构
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub goal: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub result: Option<TaskResult>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: String, goal: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            goal,
            status: TaskStatus::Pending,
            progress: 0,
            result: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 统计数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub running_tasks: usize,
    pub avg_engagement_rate: f64,
    pub avg_views: f64,
    pub avg_conversion_rate: f64,
}