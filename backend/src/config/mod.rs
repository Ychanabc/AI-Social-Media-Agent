use serde::{Deserialize, Serialize};

/// AI 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,       // openai / azure / local
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

/// 小红书 API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XhsConfig {
    pub app_id: String,
    pub app_secret: String,
}

/// 系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub ai: AiConfig,
    pub xhs: XhsConfig,
}

impl SystemConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        
        Self {
            ai: AiConfig {
                provider: std::env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string()),
                api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
                model: std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()),
                base_url: std::env::var("OPENAI_BASE_URL")
                    .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            },
            xhs: XhsConfig {
                app_id: std::env::var("XHS_APP_ID").unwrap_or_default(),
                app_secret: std::env::var("XHS_APP_SECRET").unwrap_or_default(),
            },
        }
    }

    /// 检查 AI API 是否已配置
    pub fn ai_configured(&self) -> bool {
        !self.ai.api_key.is_empty() && self.ai.api_key != "sk-your-key-here"
    }

    /// 检查小红书 API 是否已配置
    pub fn xhs_configured(&self) -> bool {
        !self.xhs.app_id.is_empty() && self.xhs.app_id != "your-app-id"
    }
}