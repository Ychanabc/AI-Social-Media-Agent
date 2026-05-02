use std::collections::HashMap;
use tokio::sync::RwLock;

use super::task::Task;
use crate::auth::AuthStore;
use crate::config::SystemConfig;
use crate::xhs::XhsService;

/// 应用全局状态（内存存储）
pub struct AppState {
    pub tasks: RwLock<HashMap<String, Task>>,
    pub auth: AuthStore,
    pub config: SystemConfig,
    pub xhs: XhsService,
}

impl AppState {
    pub fn new() -> Self {
        let config = SystemConfig::from_env();
        let xhs = XhsService::new(
            config.xhs.app_id.clone(),
            config.xhs.app_secret.clone(),
        );

        Self {
            tasks: RwLock::new(HashMap::new()),
            auth: AuthStore::new(),
            config,
            xhs,
        }
    }
}