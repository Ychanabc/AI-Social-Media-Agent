use crate::models::{ContentVersion, PublishInfo};
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

/// Publish Agent - 负责安排发布时间和平台（mock）
pub struct PublishAgent;

impl PublishAgent {
    pub async fn execute(contents: &[ContentVersion]) -> Result<Vec<PublishInfo>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

        tracing::info!("📤 PublishAgent 开始为 {} 个内容安排发布", contents.len());

        let platforms = vec!["小红书", "抖音", "微博", "微信公众号"];
        let mut publish_infos = Vec::new();

        for (i, _content) in contents.iter().enumerate() {
            let platform = platforms[i % platforms.len()];
            let now = Utc::now();
            let scheduled = now + chrono::Duration::hours((i as i64 + 1) * 2);

            publish_infos.push(PublishInfo {
                id: Uuid::new_v4().to_string(),
                platform: platform.to_string(),
                scheduled_at: scheduled.format("%Y-%m-%d %H:%M:%S").to_string(),
                status: "scheduled".to_string(),
            });
        }

        tracing::info!(
            "✅ PublishAgent 安排了 {} 个发布计划",
            publish_infos.len()
        );
        Ok(publish_infos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_publish_agent() {
        let contents = vec![ContentVersion {
            id: "test".to_string(),
            topic_id: "t1".to_string(),
            title: "测试".to_string(),
            body: "内容".to_string(),
            tone: "专业".to_string(),
            word_count: 100,
        }];
        let publish = PublishAgent::execute(&contents).await.unwrap();
        assert_eq!(publish.len(), 1);
    }
}