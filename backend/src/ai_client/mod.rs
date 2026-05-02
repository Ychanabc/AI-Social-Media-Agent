use crate::config::AiConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// OpenAI API 请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI API 响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// AI 客户端
pub struct AiClient {
    config: AiConfig,
    http: reqwest::Client,
}

impl AiClient {
    pub fn new(config: AiConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    /// 调用 AI 生成文本
    pub async fn chat(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let url = format!("{}/chat/completions", self.config.base_url);
        
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.8,
            max_tokens: 2000,
        };

        let response = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    /// 生成选题
    pub async fn generate_topics(&self, goal: &str) -> Result<String> {
        let system = "你是一个资深社交媒体运营专家，擅长为不同平台策划爆款内容选题。";
        let user = format!(
            "请为以下运营目标生成3个社媒内容选题，要求覆盖不同角度和平台风格。\n\n运营目标：{}\n\n请以JSON数组格式返回，每个选题包含：title, description, platform(小红书/抖音/微博), hashtags数组。\n只返回JSON，不要其他文字。",
            goal
        );
        self.chat(system, &user).await
    }

    /// 生成内容
    pub async fn generate_content(&self, topic_title: &str, tone: &str) -> Result<String> {
        let system = "你是一个专业的社媒内容创作者，文笔出色，善于写出引发共鸣的内容。";
        let user = format!(
            "请为以下选题撰写社媒内容。\n\n选题：{}\n风格：{}\n\n要求：\n1. 标题吸引人\n2. 正文200-500字\n3. 包含emoji增加可读性\n4. 结尾有互动引导\n\n请以JSON格式返回，包含：title, body, word_count。\n只返回JSON，不要其他文字。",
            topic_title, tone
        );
        self.chat(system, &user).await
    }
}