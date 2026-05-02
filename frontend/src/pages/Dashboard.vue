<template>
  <div class="dashboard">
    <div class="page-header">
      <h1>📊 数据总览</h1>
      <p>AI 社媒自动化系统运行概览</p>
    </div>

    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 24px">
      <el-col :span="6">
        <div class="stat-card">
          <div class="stat-icon" style="color: #409eff">📋</div>
          <div class="stat-info">
            <div class="stat-value">{{ stats?.total_tasks || 0 }}</div>
            <div class="stat-label">总任务数</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card">
          <div class="stat-icon" style="color: #67c23a">✅</div>
          <div class="stat-info">
            <div class="stat-value">{{ stats?.completed_tasks || 0 }}</div>
            <div class="stat-label">已完成</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card">
          <div class="stat-icon" style="color: #e6a23c">⏳</div>
          <div class="stat-info">
            <div class="stat-value">{{ stats?.running_tasks || 0 }}</div>
            <div class="stat-label">进行中</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card">
          <div class="stat-icon" style="color: #f56c6c">🔥</div>
          <div class="stat-info">
            <div class="stat-value">{{ stats?.avg_engagement_rate || 0 }}%</div>
            <div class="stat-label">平均互动率</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-bottom: 24px">
      <el-col :span="12">
        <div class="stat-card">
          <div class="stat-icon" style="color: #909399">👁️</div>
          <div class="stat-info">
            <div class="stat-value">{{ formatNumber(stats?.avg_views || 0) }}</div>
            <div class="stat-label">平均浏览量</div>
          </div>
        </div>
      </el-col>
      <el-col :span="12">
        <div class="stat-card">
          <div class="stat-icon" style="color: #b37feb">📈</div>
          <div class="stat-info">
            <div class="stat-value">{{ stats?.avg_conversion_rate || 0 }}%</div>
            <div class="stat-label">平均转化率</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- Pipeline 流程图 -->
    <el-card style="margin-bottom: 24px">
      <template #header>
        <span style="font-weight: 600">🔄 Agent Pipeline 流程</span>
      </template>
      <div class="pipeline-steps">
        <div class="step-card done">
          <div class="step-icon">📝</div>
          <div class="step-name">Topic</div>
          <div class="step-desc">选题生成</div>
        </div>
        <div class="step-arrow">→</div>
        <div class="step-card done">
          <div class="step-icon">✍️</div>
          <div class="step-name">Content</div>
          <div class="step-desc">内容生成</div>
        </div>
        <div class="step-arrow">→</div>
        <div class="step-card done">
          <div class="step-icon">🖼️</div>
          <div class="step-name">Media</div>
          <div class="step-desc">媒体生成</div>
        </div>
        <div class="step-arrow">→</div>
        <div class="step-card done">
          <div class="step-icon">📤</div>
          <div class="step-name">Publish</div>
          <div class="step-desc">发布安排</div>
        </div>
        <div class="step-arrow">→</div>
        <div class="step-card done">
          <div class="step-icon">📊</div>
          <div class="step-name">Analytics</div>
          <div class="step-desc">数据分析</div>
        </div>
      </div>
    </el-card>

    <!-- 快速操作 -->
    <el-card>
      <template #header>
        <span style="font-weight: 600">⚡ 快速操作</span>
      </template>
      <el-button type="primary" size="large" @click="$router.push('/generate')">
        <el-icon><MagicStick /></el-icon>
        开始生成内容
      </el-button>
      <el-button size="large" @click="$router.push('/tasks')">
        <el-icon><List /></el-icon>
        查看任务列表
      </el-button>
    </el-card>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useAppStore } from '../store'

const store = useAppStore()
const stats = computed(() => store.stats)

onMounted(() => {
  store.loadStats()
})

function formatNumber(num) {
  if (num >= 10000) return (num / 10000).toFixed(1) + 'w'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'k'
  return Math.round(num).toString()
}
</script>

<style scoped>
.stat-card {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  font-size: 36px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #1d1e1f;
}

.stat-label {
  font-size: 13px;
  color: #909399;
  margin-top: 4px;
}

.step-desc {
  font-size: 11px;
  color: #909399;
}
</style>