use crate::models::{ContentVersion, Topic};
use anyhow::Result;
use uuid::Uuid;

/// Content Agent - 负责根据选题生成内容版本
pub struct ContentAgent;

impl ContentAgent {
    pub async fn execute(topics: &[Topic]) -> Result<Vec<ContentVersion>> {
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

        tracing::info!("✍️ ContentAgent 开始为 {} 个选题生成内容", topics.len());

        let mut contents = Vec::new();
        let tones = vec!["专业严谨", "轻松幽默", "情感共鸣"];

        for topic in topics {
            for (i, tone) in tones.iter().enumerate() {
                let body = match i {
                    0 => format!(
                        "【专业分析】\n\n在当前市场环境下，{}是一个值得深入探讨的话题。\n\n通过专业数据分析和行业调研，我们发现：\n1. 用户需求持续增长\n2. 市场潜力巨大\n3. 差异化优势明显\n\n本文将从专业角度，为您深度解析{}的核心价值。",
                        topic.title, topic.title
                    ),
                    1 => format!(
                        "【轻松说】\n\n哈哈，说到{}，我可太有发言权了！\n\n先给大家讲个小故事～\n上周我亲身体验了一把，真的是...\n（此处省略一万字好评）\n\n总之就是：买它！不买后悔系列！\n\n📌 记得点赞收藏哦～",
                        topic.title
                    ),
                    _ => format!(
                        "【走心分享】\n\n有时候，生活中的小确幸就藏在细节里。\n\n关于{}，我想说：\n这不仅仅是一次体验，更是一种生活态度的表达。\n\n当我们用心去感受，会发现...\n每一处细节都值得被珍惜。\n\n🌟 愿你也能找到属于自己的那份美好。",
                        topic.title
                    ),
                };

                contents.push(ContentVersion {
                    id: Uuid::new_v4().to_string(),
                    topic_id: topic.id.clone(),
                    title: format!("{} - {}", topic.title, tone),
                    body,
                    tone: tone.to_string(),
                    word_count: 150 + i * 50,
                });
            }
        }

        tracing::info!("✅ ContentAgent 生成了 {} 个内容版本", contents.len());
        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_agent() {
        let topics = vec![Topic {
            id: "test".to_string(),
            title: "测试选题".to_string(),
            description: "测试描述".to_string(),
            platform: "小红书".to_string(),
            hashtags: vec!["测试".to_string()],
        }];
        let contents = ContentAgent::execute(&topics).await.unwrap();
        assert_eq!(contents.len(), 3);
    }
}