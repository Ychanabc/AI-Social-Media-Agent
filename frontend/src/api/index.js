import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
  headers: { 'Content-Type': 'application/json' },
})

// 请求拦截器 - 添加 token
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// 响应拦截器
api.interceptors.response.use(
  (response) => response.data,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

// ============ 认证 ============
export function login(username, password) {
  return api.post('/auth/login', { username, password })
}

export function getMe() {
  return api.get('/auth/me')
}

// ============ 管理员 ============
export function getAdminUsers() {
  return api.get('/admin/users')
}

export function createUser(data) {
  return api.post('/admin/users', data)
}

export function deleteUser(username) {
  return api.delete(`/admin/users/${username}`)
}

export function getAdminConfig() {
  return api.get('/admin/config')
}

export function updateAiConfig(data) {
  return api.post('/admin/config/ai', data)
}

// ============ 内容生成 ============
export function generateContent(goal) {
  return api.post('/generate', { goal })
}

export function getTaskStatus(taskId) {
  return api.get(`/task/${taskId}`)
}

export function getTaskList() {
  return api.get('/tasks')
}

export function getTaskResult(taskId) {
  return api.get(`/result/${taskId}`)
}

export function getStats() {
  return api.get('/stats')
}

// ============ 小红书分析 ============
export function getXhsOverview() {
  return api.get('/xhs/overview')
}

export function getXhsNotes() {
  return api.get('/xhs/notes')
}

export function getXhsNoteDetail(id) {
  return api.get(`/xhs/notes/${id}`)
}

export default api