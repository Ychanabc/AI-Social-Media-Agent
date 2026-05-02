use crate::models::AnalyticsData;
use anyhow::Result;
use rand::Rng;

/// Analytics Agent - 负责生成模拟数据分析结果
pub struct AnalyticsAgent;

impl AnalyticsAgent {
    pub async fn execute(_goal: &str) -> Result<AnalyticsData> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        tracing::info!("📊 AnalyticsAgent 开始生成分析数据");

        let mut rng = rand::thread_rng();
        let views = rng.gen_range(5000..50000);
        let likes = rng.gen_range(views / 10..views / 3);
        let shares = rng.gen_range(likes / 10..likes / 3);
        let comments = rng.gen_range(likes / 20..likes / 5);
        let conversion_rate: f64 = rng.gen_range(1.5..8.5);
        let engagement_rate: f64 = rng.gen_range(3.0..15.0);
        let sentiment_score: f64 = rng.gen_range(0.6..0.98);

        let analytics = AnalyticsData {
            views,
            likes,
            shares,
            comments,
            conversion_rate: (conversion_rate * 100.0).round() / 100.0,
            engagement_rate: (engagement_rate * 100.0).round() / 100.0,
            sentiment_score: (sentiment_score * 100.0).round() / 100.0,
        };

        tracing::info!(
            "✅ AnalyticsAgent 生成完成 | 浏览:{} 点赞:{} 互动率:{}%",
            analytics.views,
            analytics.likes,
            analytics.engagement_rate
        );

        Ok(analytics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analytics_agent() {
        let analytics = AnalyticsAgent::execute("测试目标").await.unwrap();
        assert!(analytics.views > 0);
        assert!(analytics.likes > 0);
        assert!(analytics.conversion_rate > 0.0);
    }
}