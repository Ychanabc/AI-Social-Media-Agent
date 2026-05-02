use crate::models::Topic;
use anyhow::Result;
use uuid::Uuid;

/// Topic Agent - 负责根据运营目标生成选题
pub struct TopicAgent;

impl TopicAgent {
    pub async fn execute(goal: &str) -> Result<Vec<Topic>> {
        // 模拟 AI 生成延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        tracing::info!("📝 TopicAgent 开始处理目标: {}", goal);

        let topics = vec![
            Topic {
                id: Uuid::new_v4().to_string(),
                title: format!("🔥 {} - 爆款角度解析", Self::extract_keyword(goal)),
                description: format!(
                    "从用户痛点出发，深入分析{}的核心价值，结合热点话题引发共鸣",
                    goal
                ),
                platform: "小红书".to_string(),
                hashtags: vec![
                    Self::extract_keyword(goal).to_string(),
                    "种草".to_string(),
                    "好物推荐".to_string(),
                ],
            },
            Topic {
                id: Uuid::new_v4().to_string(),
                title: format!("✨ {} - 亲测体验分享", Self::extract_keyword(goal)),
                description: format!(
                    "以第一人称视角，分享真实的{}体验过程，增强可信度",
                    goal
                ),
                platform: "抖音".to_string(),
                hashtags: vec![
                    Self::extract_keyword(goal).to_string(),
                    "测评".to_string(),
                    "真实体验".to_string(),
                ],
            },
            Topic {
                id: Uuid::new_v4().to_string(),
                title: format!("📊 {} - 数据对比指南", Self::extract_keyword(goal)),
                description: format!(
                    "通过数据对比和用户评价，展现{}的差异化优势",
                    goal
                ),
                platform: "微博".to_string(),
                hashtags: vec![
                    Self::extract_keyword(goal).to_string(),
                    "对比评测".to_string(),
                    "选购指南".to_string(),
                ],
            },
        ];

        tracing::info!("✅ TopicAgent 生成了 {} 个选题", topics.len());
        Ok(topics)
    }

    fn extract_keyword(goal: &str) -> String {
        // 简单提取关键词（mock 实现）
        if goal.len() > 10 {
            goal.chars().take(10).collect()
        } else {
            goal.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_topic_agent() {
        let topics = TopicAgent::execute("推广咖啡产品").await.unwrap();
        assert_eq!(topics.len(), 3);
        assert!(!topics[0].title.is_empty());
    }
}