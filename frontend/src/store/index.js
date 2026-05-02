import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  login as apiLogin,
  getMe,
  getAdminUsers,
  createUser,
  deleteUser,
  getAdminConfig,
  generateContent,
  getTaskStatus,
  getTaskList,
  getTaskResult,
  getStats,
  getXhsOverview,
  getXhsNotes,
} from '../api'

export const useAppStore = defineStore('app', () => {
  // 认证状态
  const token = ref(localStorage.getItem('token') || '')
  const currentUser = ref(JSON.parse(localStorage.getItem('user') || 'null'))
  const isLoggedIn = computed(() => !!token.value)
  const isAdmin = computed(() => currentUser.value?.role === 'admin')

  // 任务状态
  const tasks = ref([])
  const currentTask = ref(null)
  const currentResult = ref(null)
  const stats = ref(null)
  const loading = ref(false)
  const error = ref(null)

  const completedTasks = computed(() => tasks.value.filter((t) => t.status === 'done'))
  const runningTasks = computed(() => tasks.value.filter((t) => t.status === 'running'))

  // ============ 认证 ============
  async function login(username, password) {
    const resp = await apiLogin(username, password)
    token.value = resp.token
    currentUser.value = resp.user
    localStorage.setItem('token', resp.token)
    localStorage.setItem('user', JSON.stringify(resp.user))
    return resp
  }

  function logout() {
    token.value = ''
    currentUser.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  // ============ 管理员 ============
  async function getAdminUsersList() { return getAdminUsers() }
  async function createUserAdmin(data) { return createUser(data) }
  async function deleteUserAdmin(username) { return deleteUser(username) }
  async function getAdminConfigData() { return getAdminConfig() }

  // ============ 内容生成 ============
  async function createTask(goal) {
    loading.value = true
    error.value = null
    try {
      const response = await generateContent(goal)
      return response.task_id
    } catch (err) {
      error.value = err.message || '创建任务失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function pollTaskStatus(taskId) {
    try {
      const response = await getTaskStatus(taskId)
      currentTask.value = response
      return response
    } catch (err) {
      error.value = err.message || '查询任务状态失败'
      throw err
    }
  }

  async function loadTasks() {
    loading.value = true
    try {
      const response = await getTaskList()
      tasks.value = response.tasks || []
    } catch (err) {
      error.value = err.message || '加载任务列表失败'
    } finally {
      loading.value = false
    }
  }

  async function loadResult(taskId) {
    loading.value = true
    error.value = null
    try {
      const response = await getTaskResult(taskId)
      currentResult.value = response
      return response
    } catch (err) {
      if (err.response?.status === 202) error.value = '任务尚未完成'
      else error.value = err.message || '加载结果失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function loadStats() {
    try { stats.value = await getStats() } catch (err) { console.error(err) }
  }

  // ============ 小红书分析 ============
  async function getXhsOverviewData() { return getXhsOverview() }
  async function getXhsNotesData() { return getXhsNotes() }

  function reset() {
    currentTask.value = null
    currentResult.value = null
    error.value = null
  }

  return {
    token, currentUser, isLoggedIn, isAdmin,
    tasks, currentTask, currentResult, stats, loading, error,
    completedTasks, runningTasks,
    login, logout,
    getAdminUsers: getAdminUsersList, createUser: createUserAdmin, deleteUser: deleteUserAdmin, getAdminConfig: getAdminConfigData,
    createTask, pollTaskStatus, loadTasks, loadResult, loadStats,
    getXhsOverview: getXhsOverviewData, getXhsNotes: getXhsNotesData,
    reset,
  }
})