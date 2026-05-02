你是一个资深全栈架构师 + AI Agent 系统工程师。

目标：
构建一个「AI 社媒自动化系统 Demo」，要求：

【后端】
- 使用 Rust 实现（Axum + Tokio）
- 提供 REST API
- 实现一个最小可运行的 Multi-Agent Pipeline：
  Topic → Content → Media → Publish → Analytics

【前端】
- 使用 Vue 3 + Vite
- 实现一个管理后台（Dashboard）

【系统要求】
- 可本地运行（docker 可选）
- 模块清晰（可扩展）
- 使用 mock 数据代替真实平台 API
- 所有流程可在 UI 上触发

【核心能力】
- 用户输入“运营目标”
- 系统自动生成内容
- 展示生成结果
- 模拟数据分析结果

请输出：
1. 项目目录结构
2. 后端模块划分
3. API 设计（接口定义）
4. 前端页面结构
5. Agent 执行流程
6. 数据结构（JSON Schema）
7. 本地运行步骤

要求：
- 结构清晰
- 可直接用于开发
- 避免伪代码，使用工程描述
🦀 二、Rust 后端开发提示词
你是一个 Rust 后端架构师。

请设计一个高性能 API 服务，用于 AI Agent 社媒系统。

技术要求：
- 框架：Axum
- 异步：Tokio
- 数据格式：JSON
- 日志：tracing
- 错误处理：anyhow

功能模块：

1. API 层
- POST /generate
- GET /tasks
- GET /result/{id}

2. Agent 模块
- TopicAgent
- ContentAgent
- MediaAgent
- PublishAgent
- AnalyticsAgent

3. Orchestrator
- 串联 Agent 执行
- 支持 trace_id
- 支持并发执行（content + media）

4. 存储
- 使用内存存储（HashMap）模拟数据库
- Redis 可选

5. Task 系统
- 每个请求生成 task_id
- 支持状态：
  pending / running / done

输出内容：
- 模块划分说明
- 每个模块职责
- 数据流说明
- 并发策略说明
⚙️ 三、Agent 实现提示词（核心）
你是 AI Agent 系统设计专家。

请设计一个最小可运行的 Agent Pipeline：

流程：
Topic → Content → Media → Publish → Analytics

要求：

1. 每个 Agent 是独立模块
2. 输入输出统一为 JSON
3. 每个 Agent 必须可单独测试
4. 支持 mock 数据

具体要求：

Topic Agent：
- 输入：用户目标
- 输出：3个选题

Content Agent：
- 输入：选题
- 输出：3个内容版本

Media Agent：
- 输出：图片 URL（mock）

Publish Agent：
- 输出：发布时间（mock）

Analytics Agent：
- 输出：模拟数据：
  - views
  - likes
  - conversion_rate

最终输出：
完整链路 JSON：
{
  "topic": ...,
  "content": ...,
  "media": ...,
  "publish": ...,
  "analytics": ...
}
🌐 四、API 设计提示词
请设计 RESTful API，用于前端调用。

要求：

1. 创建任务
POST /api/generate
body:
{
  "goal": "推广一款咖啡产品"
}

response:
{
  "task_id": "xxx"
}

2. 查询任务状态
GET /api/task/{id}

response:
{
  "status": "running | done",
  "progress": 0-100
}

3. 获取结果
GET /api/result/{id}

response:
{
  "topic": [...],
  "content": [...],
  "media": {...},
  "analytics": {...}
}

要求：
- JSON 标准化
- 字段命名统一
- 可扩展
🖥️ 五、Vue 前端开发提示词
你是一个 Vue 3 前端架构师。

请设计一个 AI 社媒系统管理后台 UI。

技术要求：
- Vue 3 + Vite
- 状态管理：Pinia
- UI：Element Plus 或 Naive UI
- 请求：Axios

页面要求：

1. 首页 Dashboard
- 展示任务数量
- 平均互动率（mock）

2. 内容生成页（核心）
- 输入框（运营目标）
- “生成内容”按钮
- 展示：
  - 选题
  - 内容版本
  - 图片
  - 数据分析

3. 任务列表页
- 展示所有 task
- 状态标识

4. 结果详情页
- 展示完整 Agent 输出

交互要求：
- 点击生成 → 调用 API
- 轮询任务状态
- 自动刷新结果

输出：
- 页面结构
- 组件拆分
- 状态流设计
🔄 六、前后端联调提示词
请设计前后端联调方案。

要求：

1. 前端调用流程：
- 用户输入 → POST /generate
- 轮询 GET /task/{id}
- 完成后调用 /result/{id}

2. 错误处理：
- loading 状态
- 超时处理
- 重试机制

3. 数据展示：
- JSON → UI 映射

4. 本地开发：
- 前端运行在 5173
- 后端运行在 3000
- 解决跨域问题（CORS）

输出：
- 调用流程图（文字）
- 状态流说明
🧩 七、项目结构生成提示词
请生成一个完整项目结构：

要求：

backend/
- src/
  - main.rs
  - api/
  - agents/
  - orchestrator/
  - models/

frontend/
- src/
  - pages/
  - components/
  - store/
  - api/

要求：
- 清晰分层
- 可扩展
- 符合最佳实践
🚀 八、运行与交付提示词
请输出项目运行说明：

要求：

后端：
- cargo run
- 环境变量说明

前端：
- npm install
- npm run dev

可选：
- docker-compose

输出：
- 一步步运行指南
- 常见错误说明