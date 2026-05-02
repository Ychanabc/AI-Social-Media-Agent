use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub display_name: String,
    pub created_at: String,
}

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

/// 用户信息（不含密码）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: Role,
    pub display_name: String,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: Role,
    pub display_name: String,
}

/// 用户管理存储
pub struct AuthStore {
    pub users: RwLock<HashMap<String, User>>,
    pub tokens: RwLock<HashMap<String, String>>, // token -> user_id
}

impl AuthStore {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        // 默认管理员账号
        users.insert(
            "admin".to_string(),
            User {
                id: "admin-001".to_string(),
                username: "admin".to_string(),
                password_hash: hash_password("admin123"),
                role: Role::Admin,
                display_name: "系统管理员".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        );
        // 默认普通用户
        users.insert(
            "user".to_string(),
            User {
                id: "user-001".to_string(),
                username: "user".to_string(),
                password_hash: hash_password("user123"),
                role: Role::User,
                display_name: "运营人员".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        );

        Self {
            users: RwLock::new(users),
            tokens: RwLock::new(HashMap::new()),
        }
    }

    /// 登录
    pub async fn login(&self, req: &LoginRequest) -> Option<LoginResponse> {
        let users = self.users.read().await;
        let user = users.get(&req.username)?;
        
        if user.password_hash != hash_password(&req.password) {
            return None;
        }

        let token = Uuid::new_v4().to_string();
        let mut tokens = self.tokens.write().await;
        tokens.insert(token.clone(), user.id.clone());

        Some(LoginResponse {
            token,
            user: UserInfo {
                id: user.id.clone(),
                username: user.username.clone(),
                role: user.role.clone(),
                display_name: user.display_name.clone(),
            },
        })
    }

    /// 验证 token
    pub async fn verify_token(&self, token: &str) -> Option<UserInfo> {
        let tokens = self.tokens.read().await;
        let user_id = tokens.get(token)?;
        
        let users = self.users.read().await;
        users.values().find(|u| &u.id == user_id).map(|u| UserInfo {
            id: u.id.clone(),
            username: u.username.clone(),
            role: u.role.clone(),
            display_name: u.display_name.clone(),
        })
    }

    /// 获取所有用户
    pub async fn list_users(&self) -> Vec<UserInfo> {
        let users = self.users.read().await;
        users
            .values()
            .map(|u| UserInfo {
                id: u.id.clone(),
                username: u.username.clone(),
                role: u.role.clone(),
                display_name: u.display_name.clone(),
            })
            .collect()
    }

    /// 创建用户
    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserInfo, String> {
        let mut users = self.users.write().await;
        if users.contains_key(&req.username) {
            return Err("用户名已存在".to_string());
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            username: req.username.clone(),
            password_hash: hash_password(&req.password),
            role: req.role,
            display_name: req.display_name,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let info = UserInfo {
            id: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            display_name: user.display_name.clone(),
        };

        users.insert(req.username, user);
        Ok(info)
    }

    /// 删除用户
    pub async fn delete_user(&self, username: &str) -> Result<(), String> {
        let mut users = self.users.write().await;
        if username == "admin" {
            return Err("不能删除默认管理员".to_string());
        }
        users.remove(username).ok_or("用户不存在".to_string())?;
        Ok(())
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}