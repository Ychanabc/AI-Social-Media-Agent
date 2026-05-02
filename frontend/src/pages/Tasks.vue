<template>
  <div class="tasks-page">
    <div class="page-header">
      <h1>📋 任务列表</h1>
      <p>查看所有 AI Agent 执行任务</p>
    </div>

    <el-card>
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span style="font-weight: 600">所有任务 ({{ tasks.length }})</span>
          <el-button @click="refreshTasks" :loading="loading">
            <el-icon><Refresh /></el-icon>
            刷新
          </el-button>
        </div>
      </template>

      <el-table :data="tasks" stripe v-loading="loading" style="width: 100%">
        <el-table-column prop="task_id" label="任务 ID" width="300">
          <template #default="{ row }">
            <el-text type="primary" style="cursor: pointer" @click="goToResult(row.task_id)">
              {{ row.task_id }}
            </el-text>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="120">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="default">
              {{ statusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="progress" label="进度" width="200">
          <template #default="{ row }">
            <el-progress
              :percentage="row.progress"
              :status="row.status === 'done' ? 'success' : undefined"
              :stroke-width="12"
            />
          </template>
        </el-table-column>

        <el-table-column prop="created_at" label="创建时间" width="200">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>

        <el-table-column prop="updated_at" label="更新时间" width="200">
          <template #default="{ row }">
            {{ formatTime(row.updated_at) }}
          </template>
        </el-table-column>

        <el-table-column label="操作" width="120">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'done'"
              type="primary"
              size="small"
              @click="goToResult(row.task_id)"
            >
              查看结果
            </el-button>
            <el-button
              v-else-if="row.status === 'running'"
              type="warning"
              size="small"
              disabled
            >
              执行中
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <div v-if="tasks.length === 0 && !loading" style="text-align: center; padding: 60px 0">
        <el-empty description="暂无任务">
          <el-button type="primary" @click="$router.push('/generate')">
            创建第一个任务
          </el-button>
        </el-empty>
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAppStore } from '../store'

const store = useAppStore()
const router = useRouter()
const refreshTimer = ref(null)

const tasks = computed(() => store.tasks)
const loading = computed(() => store.loading)

onMounted(() => {
  store.loadTasks()
  // 自动刷新
  refreshTimer.value = setInterval(() => {
    store.loadTasks()
  }, 3000)
})

onUnmounted(() => {
  if (refreshTimer.value) {
    clearInterval(refreshTimer.value)
  }
})

function refreshTasks() {
  store.loadTasks()
}

function goToResult(taskId) {
  router.push(`/result/${taskId}`)
}

function statusType(status) {
  const map = { pending: 'info', running: 'warning', done: 'success', failed: 'danger' }
  return map[status] || 'info'
}

function statusText(status) {
  const map = { pending: '等待中', running: '执行中', done: '已完成', failed: '失败' }
  return map[status] || status
}

function formatTime(timeStr) {
  if (!timeStr) return '-'
  const date = new Date(timeStr)
  return date.toLocaleString('zh-CN')
}
</script>