<template>
  <div class="generate-page">
    <div class="page-header">
      <h1>✨ 内容生成</h1>
      <p>输入运营目标，AI Agent 自动生成完整内容方案</p>
    </div>

    <!-- 输入区域 -->
    <el-card style="margin-bottom: 24px">
      <el-input
        v-model="goal"
        type="textarea"
        :rows="3"
        placeholder="请输入运营目标，例如：推广一款新的咖啡产品，目标用户是25-35岁的都市白领"
        :disabled="isGenerating"
      />
      <div style="margin-top: 16px; display: flex; gap: 12px">
        <el-button
          type="primary"
          size="large"
          :loading="isGenerating"
          @click="handleGenerate"
        >
          <el-icon v-if="!isGenerating"><MagicStick /></el-icon>
          {{ isGenerating ? '生成中...' : '🚀 开始生成' }}
        </el-button>
        <el-button size="large" @click="handleReset" :disabled="isGenerating">
          重置
        </el-button>
      </div>

      <!-- 快捷模板 -->
      <div style="margin-top: 12px">
        <span style="font-size: 13px; color: #909399; margin-right: 8px">快捷模板:</span>
        <el-tag
          v-for="tpl in templates"
          :key="tpl"
          style="cursor: pointer; margin-right: 8px"
          @click="goal = tpl"
        >
          {{ tpl }}
        </el-tag>
      </div>
    </el-card>

    <!-- Pipeline 进度 -->
    <el-card v-if="currentTask" style="margin-bottom: 24px">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span style="font-weight: 600">🔄 Agent Pipeline 执行进度</span>
          <el-tag :type="statusType">{{ statusText }}</el-tag>
        </div>
      </template>

      <el-progress
        :percentage="currentTask.progress"
        :status="currentTask.status === 'done' ? 'success' : undefined"
        :stroke-width="20"
        style="margin-bottom: 20px"
      />

      <div class="pipeline-steps">
        <div
          v-for="(step, index) in pipelineSteps"
          :key="step.name"
          class="pipeline-step"
        >
          <div
            class="step-card"
            :class="{
              active: isStepActive(index),
              done: isStepDone(index),
            }"
          >
            <div class="step-icon">{{ step.icon }}</div>
            <div class="step-name">{{ step.name }}</div>
          </div>
          <div v-if="index < pipelineSteps.length - 1" class="step-arrow">
            →
          </div>
        </div>
      </div>
    </el-card>

    <!-- 结果展示 -->
    <template v-if="result">
      <!-- 选题结果 -->
      <el-card class="result-section" style="margin-bottom: 24px">
        <template #header>
          <div class="section-title">📝 选题方案 ({{ result.topics.length }} 个)</div>
        </template>
        <el-row :gutter="16">
          <el-col :span="8" v-for="topic in result.topics" :key="topic.id">
            <el-card shadow="hover">
              <h3 style="font-size: 15px; margin-bottom: 8px">{{ topic.title }}</h3>
              <p style="font-size: 13px; color: #606266; margin-bottom: 12px">
                {{ topic.description }}
              </p>
              <div style="display: flex; justify-content: space-between; align-items: center">
                <el-tag size="small" type="info">{{ topic.platform }}</el-tag>
                <div>
                  <el-tag
                    v-for="tag in topic.hashtags"
                    :key="tag"
                    size="small"
                    type="warning"
                    style="margin-left: 4px"
                  >
                    #{{ tag }}
                  </el-tag>
                </div>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </el-card>

      <!-- 内容版本 -->
      <el-card class="result-section" style="margin-bottom: 24px">
        <template #header>
          <div class="section-title">✍️ 内容版本 ({{ result.contents.length }} 个)</div>
        </template>
        <el-collapse>
          <el-collapse-item
            v-for="content in result.contents"
            :key="content.id"
            :title="content.title"
            :name="content.id"
          >
            <div style="padding: 12px">
              <div style="margin-bottom: 8px">
                <el-tag size="small" :type="toneType(content.tone)">{{ content.tone }}</el-tag>
                <el-tag size="small" style="margin-left: 8px">{{ content.word_count }} 字</el-tag>
              </div>
              <div style="white-space: pre-wrap; line-height: 1.8; color: #303133">
                {{ content.body }}
              </div>
            </div>
          </el-collapse-item>
        </el-collapse>
      </el-card>

      <!-- 媒体资源 -->
      <el-card class="result-section" style="margin-bottom: 24px">
        <template #header>
          <div class="section-title">🖼️ 媒体资源 ({{ result.media.length }} 个)</div>
        </template>
        <el-row :gutter="16">
          <el-col :span="6" v-for="media in result.media" :key="media.id">
            <el-card shadow="hover" :body-style="{ padding: '0' }">
              <img
                :src="media.url"
                style="width: 100%; height: 160px; object-fit: cover"
                alt="媒体资源"
              />
              <div style="padding: 12px">
                <el-tag size="small">{{ media.media_type }}</el-tag>
                <span style="font-size: 12px; color: #909399; margin-left: 8px">
                  {{ media.width }}×{{ media.height }}
                </span>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </el-card>

      <!-- 发布计划 -->
      <el-card class="result-section" style="margin-bottom: 24px">
        <template #header>
          <div class="section-title">📤 发布计划</div>
        </template>
        <el-table :data="result.publish" stripe>
          <el-table-column prop="platform" label="平台" width="150" />
          <el-table-column prop="scheduled_at" label="发布时间" width="200" />
          <el-table-column prop="status" label="状态" width="120">
            <template #default="{ row }">
              <el-tag :type="row.status === 'scheduled' ? 'success' : 'info'" size="small">
                {{ row.status }}
              </el-tag>
            </template>
          </el-table-column>
        </el-table>
      </el-card>

      <!-- 数据分析 -->
      <el-card class="result-section" style="margin-bottom: 24px">
        <template #header>
          <div class="section-title">📊 数据分析预测</div>
        </template>
        <el-row :gutter="16">
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ formatNumber(result.analytics.views) }}</div>
              <div class="analytic-label">预计浏览量</div>
            </div>
          </el-col>
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ formatNumber(result.analytics.likes) }}</div>
              <div class="analytic-label">预计点赞</div>
            </div>
          </el-col>
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ formatNumber(result.analytics.shares) }}</div>
              <div class="analytic-label">预计分享</div>
            </div>
          </el-col>
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ formatNumber(result.analytics.comments) }}</div>
              <div class="analytic-label">预计评论</div>
            </div>
          </el-col>
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ result.analytics.conversion_rate }}%</div>
              <div class="analytic-label">转化率</div>
            </div>
          </el-col>
          <el-col :span="4">
            <div class="analytic-item">
              <div class="analytic-value">{{ result.analytics.engagement_rate }}%</div>
              <div class="analytic-label">互动率</div>
            </div>
          </el-col>
        </el-row>
      </el-card>
    </template>
  </div>
</template>

<script setup>
import { ref, computed, onUnmounted } from 'vue'
import { useAppStore } from '../store'
import { ElMessage } from 'element-plus'

const store = useAppStore()

const goal = ref('')
const isGenerating = ref(false)
const pollTimer = ref(null)

const currentTask = computed(() => store.currentTask)
const result = computed(() => store.currentResult)

const templates = [
  '推广一款咖啡产品',
  '新品手机上市宣传',
  '健身App用户增长',
  '旅游目的地种草',
]

const pipelineSteps = [
  { name: 'Topic', icon: '📝' },
  { name: 'Content', icon: '✍️' },
  { name: 'Media', icon: '🖼️' },
  { name: 'Publish', icon: '📤' },
  { name: 'Analytics', icon: '📊' },
]

const statusType = computed(() => {
  if (!currentTask.value) return 'info'
  const map = { pending: 'info', running: 'warning', done: 'success', failed: 'danger' }
  return map[currentTask.value.status] || 'info'
})

const statusText = computed(() => {
  if (!currentTask.value) return ''
  const map = { pending: '等待中', running: '执行中', done: '已完成', failed: '失败' }
  return map[currentTask.value.status] || ''
})

function isStepActive(index) {
  if (!currentTask.value) return false
  const progress = currentTask.value.progress
  const stepProgress = (index + 1) * 20
  return progress >= stepProgress - 20 && progress < stepProgress
}

function isStepDone(index) {
  if (!currentTask.value) return false
  const progress = currentTask.value.progress
  return progress >= (index + 1) * 20
}

function toneType(tone) {
  const map = { '专业严谨': '', '轻松幽默': 'success', '情感共鸣': 'warning' }
  return map[tone] || 'info'
}

function formatNumber(num) {
  if (num >= 10000) return (num / 10000).toFixed(1) + 'w'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'k'
  return Math.round(num).toString()
}

async function handleGenerate() {
  if (!goal.value.trim()) {
    ElMessage.warning('请输入运营目标')
    return
  }

  isGenerating.value = true
  store.reset()

  try {
    const taskId = await store.createTask(goal.value)
    ElMessage.success(`任务已创建: ${taskId.slice(0, 8)}...`)

    // 开始轮询
    startPolling(taskId)
  } catch (err) {
    ElMessage.error('创建任务失败')
    isGenerating.value = false
  }
}

function startPolling(taskId) {
  pollTimer.value = setInterval(async () => {
    try {
      const status = await store.pollTaskStatus(taskId)

      if (status.status === 'done') {
        clearInterval(pollTimer.value)
        isGenerating.value = false
        ElMessage.success('🎉 内容生成完成！')

        // 加载结果
        await store.loadResult(taskId)
      } else if (status.status === 'failed') {
        clearInterval(pollTimer.value)
        isGenerating.value = false
        ElMessage.error('任务执行失败')
      }
    } catch (err) {
      // 继续轮询
    }
  }, 800)
}

function handleReset() {
  if (pollTimer.value) {
    clearInterval(pollTimer.value)
  }
  goal.value = ''
  isGenerating.value = false
  store.reset()
}

onUnmounted(() => {
  if (pollTimer.value) {
    clearInterval(pollTimer.value)
  }
})
</script>

<style scoped>
.analytic-item {
  text-align: center;
  padding: 20px 0;
}

.analytic-value {
  font-size: 24px;
  font-weight: 700;
  color: #409eff;
}

.analytic-label {
  font-size: 13px;
  color: #909399;
  margin-top: 8px;
}
</style>