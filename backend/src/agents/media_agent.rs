use crate::models::{ContentVersion, MediaAsset, Topic};
use anyhow::Result;
use uuid::Uuid;

const MOCK_URLS: &[&str] = &[
    "https://images.unsplash.com/photo-1495474472287-4d71bcdd2085?w=800",
    "https://images.unsplash.com/photo-1509042239860-f550ce710b93?w=800",
    "https://images.unsplash.com/photo-1442512595331-e89e73853f31?w=800",
    "https://images.unsplash.com/photo-1504630083234-14187a9df0f5?w=800",
    "https://images.unsplash.com/photo-1461023058943-07fcbe16d735?w=800",
    "https://images.unsplash.com/photo-1485808191679-5f86510681a2?w=800",
];

/// Media Agent - 负责为内容生成配图资源（mock）
pub struct MediaAgent;

impl MediaAgent {
    /// 基于已生成的内容生成媒体资源
    pub async fn execute(contents: &[ContentVersion]) -> Result<Vec<MediaAsset>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;

        tracing::info!("🖼️ MediaAgent 开始为 {} 个内容生成媒体资源", contents.len());

        let media_assets: Vec<MediaAsset> = contents
            .iter()
            .enumerate()
            .map(|(i, content)| MediaAsset {
                id: Uuid::new_v4().to_string(),
                content_id: content.id.clone(),
                url: MOCK_URLS[i % MOCK_URLS.len()].to_string(),
                media_type: "image/jpeg".to_string(),
                width: 800,
                height: 600,
            })
            .collect();

        tracing::info!("✅ MediaAgent 生成了 {} 个媒体资源", media_assets.len());
        Ok(media_assets)
    }

    /// 基于选题生成媒体资源（可与 ContentAgent 并发执行）
    pub async fn execute_for_topics(topics: &[Topic]) -> Result<Vec<MediaAsset>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;

        tracing::info!("🖼️ MediaAgent 开始为 {} 个选题生成媒体资源", topics.len());

        let media_assets: Vec<MediaAsset> = topics
            .iter()
            .enumerate()
            .map(|(i, topic)| MediaAsset {
                id: Uuid::new_v4().to_string(),
                content_id: topic.id.clone(),
                url: MOCK_URLS[i % MOCK_URLS.len()].to_string(),
                media_type: "image/jpeg".to_string(),
                width: 800,
                height: 600,
            })
            .collect();

        tracing::info!("✅ MediaAgent 生成了 {} 个媒体资源", media_assets.len());
        Ok(media_assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_media_agent() {
        let contents = vec![ContentVersion {
            id: "test".to_string(),
            topic_id: "t1".to_string(),
            title: "测试".to_string(),
            body: "内容".to_string(),
            tone: "专业".to_string(),
            word_count: 100,
        }];
        let media = MediaAgent::execute(&contents).await.unwrap();
        assert_eq!(media.len(), 1);
    }
}