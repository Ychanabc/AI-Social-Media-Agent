# 🤖 AI 社媒自动化系统 (AI Social Media Agent)

> 🎯 基于 Multi-Agent Pipeline 的 AI 社交媒体内容自动化生成系统 Demo

一个开源 Demo 项目，展示如何用 AI Agent 架构实现社媒运营全链路自动化：从选题、内容生成、素材匹配、发布安排到数据分析。

## ✨ 核心功能

- **🧠 AI 内容生成** - 输入运营目标，5 个 Agent 自动串联生成完整内容方案
- **📊 小红书数据分析** - 接入小红书开放平台 API，实时追踪笔记表现
- **👥 多角色管理** - 管理员/用户双角色，权限隔离
- **⚙️ AI 接口配置** - 支持 OpenAI 兼容 API（可对接任何 LLM 服务）
- **📱 多平台运营** - 覆盖小红书、抖音、微博、微信公众号

## 🏗️ 技术栈

| 层级 | 技术 |
|------|------|
| 后端 | Rust + Axum + Tokio |
| 前端 | Vue 3 + Vite + Element Plus + Pinia |
| 通信 | REST API (JSON) |
| 存储 | 内存 HashMap（可扩展至 Redis/DB） |
| AI | OpenAI 兼容 API |
| 数据源 | 小红书开放平台 API |

## 🔄 Agent Pipeline 流程

```
用户输入目标 → Topic Agent → Content Agent → Media Agent → Publish Agent → Analytics Agent
                   ↓              ↓              ↓              ↓              ↓
               3个选题       9个内容版本      3个媒体资源    9个发布计划    数据分析预测
```

| Agent | 输入 | 输出 | 说明 |
|-------|------|------|------|
| **TopicAgent** | 运营目标 | 3 个选题 | 根据目标生成多平台选题方案 |
| **ContentAgent** | 选题列表 | 9 个内容版本 | 每个选题生成 3 种风格（专业/幽默/情感） |
| **MediaAgent** | 选题列表 | 3 个媒体资源 | 匹配配图资源（支持并发执行） |
| **PublishAgent** | 内容列表 | 9 个发布计划 | 安排多平台发布时间 |
| **AnalyticsAgent** | 运营目标 | 分析数据 | 生成浏览/点赞/转化等指标预测 |

## 📁 项目结构

```
AI-Social-Media-Agent/
├── backend/                          # Rust 后端
│   ├── Cargo.toml                   # 依赖配置
│   ├── .env.example                 # 环境变量模板
│   ├── Dockerfile
│   └── src/
│       ├── main.rs                  # 入口：Axum 服务启动
│       ├── api/handlers.rs          # REST API 路由处理器
│       ├── agents/                  # 5 个 Agent 模块
│       │   ├── topic_agent.rs       # 选题生成
│       │   ├── content_agent.rs     # 内容生成
│       │   ├── media_agent.rs       # 媒体匹配
│       │   ├── publish_agent.rs     # 发布安排
│       │   └── analytics_agent.rs   # 数据分析
│       ├── ai_client/mod.rs         # OpenAI 兼容 API 客户端
│       ├── auth/mod.rs              # 用户认证与角色系统
│       ├── config/mod.rs            # 系统配置管理
│       ├── models/                  # 数据模型
│       ├── orchestrator/mod.rs      # Pipeline 编排器
│       └── xhs/mod.rs               # 小红书数据分析模块
├── frontend/                         # Vue 3 前端
│   ├── package.json
│   ├── vite.config.js
│   ├── Dockerfile
│   └── src/
│       ├── App.vue                  # 根组件（侧边栏导航）
│       ├── api/index.js             # Axios API 封装（含 Token 拦截）
│       ├── store/index.js           # Pinia 状态管理
│       ├── router/index.js          # 路由（含登录守卫）
│       └── pages/
│           ├── Login.vue            # 登录页
│           ├── Dashboard.vue        # 数据总览
│           ├── Generate.vue         # 内容生成（核心）
│           ├── Tasks.vue            # 任务列表
│           ├── Result.vue           # 结果详情
│           ├── XhsAnalytics.vue     # 小红书数据分析
│           └── Admin.vue            # 管理后台
├── docker-compose.yml
└── README.md
```

## 🌐 API 设计

### 认证
| 方法 | 路径 | 说明 | 权限 |
|------|------|------|------|
| POST | `/api/auth/login` | 用户登录 | 公开 |
| GET | `/api/auth/me` | 获取当前用户 | 登录 |

### 管理员
| 方法 | 路径 | 说明 | 权限 |
|------|------|------|------|
| GET | `/api/admin/users` | 用户列表 | Admin |
| POST | `/api/admin/users` | 创建用户 | Admin |
| DELETE | `/api/admin/users/:username` | 删除用户 | Admin |
| GET | `/api/admin/config` | 系统配置 | Admin |

### 内容生成
| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/generate` | 创建生成任务 |
| GET | `/api/task/:id` | 查询任务状态 |
| GET | `/api/tasks` | 任务列表 |
| GET | `/api/result/:id` | 获取完整结果 |
| GET | `/api/stats` | 统计数据 |

### 小红书分析
| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/xhs/overview` | 数据概览 |
| GET | `/api/xhs/notes` | 笔记列表 |
| GET | `/api/xhs/notes/:id` | 笔记详情 |

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/Ychanabc/AI-Social-Media-Agent.git
cd AI-Social-Media-Agent
```

### 2. 配置环境变量（可选）

```bash
cp backend/.env.example backend/.env
# 编辑 backend/.env，填入你的 API Key
```

### 3. 启动后端

```bash
cd backend
cargo run
# 后端将在 http://localhost:3000 启动
```

### 4. 启动前端（新终端）

```bash
cd frontend
npm install
npm run dev
# 前端将在 http://localhost:5173 启动
```

### 5. 登录使用

打开 http://localhost:5173，使用以下账号登录：

| 账号 | 密码 | 角色 |
|------|------|------|
| admin | admin123 | 管理员（可访问管理后台） |
| user | user123 | 普通用户 |

### Docker 部署（可选）

```bash
docker-compose up --build
```

## ⚙️ AI 接口配置

编辑 `backend/.env` 文件：

```env
# OpenAI 兼容 API（支持任何兼容服务）
AI_PROVIDER=openai
OPENAI_API_KEY=sk-your-key-here
OPENAI_MODEL=gpt-4o-mini
OPENAI_BASE_URL=https://api.openai.com/v1
```

支持的 AI 服务：
- OpenAI（GPT-4o / GPT-4o-mini）
- Azure OpenAI
- DeepSeek
- 任何 OpenAI 兼容 API

## 📱 小红书 API 配置

1. 前往 [小红书开放平台](https://open.xiaohongshu.com) 注册开发者账号
2. 创建应用，获取 App ID 和 App Secret
3. 编辑 `backend/.env`：

```env
XHS_APP_ID=your-app-id
XHS_APP_SECRET=your-app-secret
```

当前使用 mock 数据演示，配置真实 API 后自动切换。

## 📄 License

MIT

## 🤝 Contributing

欢迎提 Issue 和 PR！