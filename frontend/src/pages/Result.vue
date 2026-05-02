<template>
  <div class="result-page">
    <div class="page-header">
      <div style="display: flex; justify-content: space-between; align-items: center">
        <div>
          <h1>📄 结果详情</h1>
          <p>任务 ID: {{ taskId }}</p>
        </div>
        <el-button @click="$router.push('/tasks')">
          <el-icon><ArrowLeft /></el-icon>
          返回列表
        </el-button>
      </div>
    </div>

    <div v-loading="loading">
      <template v-if="result">
        <!-- 任务信息 -->
        <el-card style="margin-bottom: 24px">
          <el-descriptions title="任务信息" :column="3" border>
            <el-descriptions-item label="运营目标">
              {{ result.goal }}
            </el-descriptions-item>
            <el-descriptions-item label="任务 ID">
              {{ result.task_id }}
            </el-descriptions-item>
            <el-descriptions-item label="状态">
              <el-tag type="success">已完成</el-tag>
            </el-descriptions-item>
          </el-descriptions>
        </el-card>

        <!-- 选题 -->
        <el-card style="margin-bottom: 24px">
          <template #header>
            <span style="font-weight: 600">📝 选题方案</span>
          </template>
          <el-row :gutter="16">
            <el-col :span="8" v-for="topic in result.topics" :key="topic.id">
              <el-card shadow="hover" style="margin-bottom: 16px">
                <h3 style="margin-bottom: 8px">{{ topic.title }}</h3>
                <p style="font-size: 13px; color: #606266; margin-bottom: 12px">
                  {{ topic.description }}
                </p>
                <div>
                  <el-tag size="small" type="info">{{ topic.platform }}</el-tag>
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
              </el-card>
            </el-col>
          </el-row>
        </el-card>

        <!-- 内容 -->
        <el-card style="margin-bottom: 24px">
          <template #header>
            <span style="font-weight: 600">✍️ 内容版本</span>
          </template>
          <el-tabs>
            <el-tab-pane
              v-for="content in result.contents"
              :key="content.id"
              :label="content.tone"
            >
              <h3 style="margin-bottom: 12px">{{ content.title }}</h3>
              <div style="white-space: pre-wrap; line-height: 1.8; padding: 16px; background: #f5f7fa; border-radius: 8px">
                {{ content.body }}
              </div>
              <div style="margin-top: 12px">
                <el-tag size="small">{{ content.word_count }} 字</el-tag>
                <el-tag size="small" type="info" style="margin-left: 4px">{{ content.tone }}</el-tag>
              </div>
            </el-tab-pane>
          </el-tabs>
        </el-card>

        <!-- 媒体 -->
        <el-card style="margin-bottom: 24px">
          <template #header>
            <span style="font-weight: 600">🖼️ 媒体资源</span>
          </template>
          <el-row :gutter="16">
            <el-col :span="6" v-for="media in result.media" :key="media.id">
              <el-card shadow="hover" :body-style="{ padding: '0' }" style="margin-bottom: 16px">
                <img
                  :src="media.url"
                  style="width: 100%; height: 160px; object-fit: cover"
                  alt="media"
                />
                <div style="padding: 12px">
                  <el-tag size="small">{{ media.media_type }}</el-tag>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </el-card>

        <!-- 发布计划 -->
        <el-card style="margin-bottom: 24px">
          <template #header>
            <span style="font-weight: 600">📤 发布计划</span>
          </template>
          <el-table :data="result.publish" stripe>
            <el-table-column prop="platform" label="平台" />
            <el-table-column prop="scheduled_at" label="发布时间" />
            <el-table-column prop="status" label="状态">
              <template #default="{ row }">
                <el-tag type="success" size="small">{{ row.status }}</el-tag>
              </template>
            </el-table-column>
          </el-table>
        </el-card>

        <!-- 分析数据 -->
        <el-card style="margin-bottom: 24px">
          <template #header>
            <span style="font-weight: 600">📊 数据分析</span>
          </template>
          <el-row :gutter="20">
            <el-col :span="4" v-for="item in analyticsItems" :key="item.label">
              <div style="text-align: center; padding: 20px 0">
                <div style="font-size: 28px; font-weight: 700; color: #409eff">
                  {{ item.value }}
                </div>
                <div style="font-size: 13px; color: #909399; margin-top: 8px">
                  {{ item.label }}
                </div>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </template>

      <el-empty v-else-if="!loading" description="未找到结果">
        <el-button type="primary" @click="$router.push('/generate')">
          去生成内容
        </el-button>
      </el-empty>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useAppStore } from '../store'

const route = useRoute()
const store = useAppStore()

const taskId = computed(() => route.params.id)
const result = computed(() => store.currentResult)
const loading = computed(() => store.loading)

const analyticsItems = computed(() => {
  if (!result.value) return []
  const a = result.value.analytics
  return [
    { label: '预计浏览量', value: formatNum(a.views) },
    { label: '预计点赞', value: formatNum(a.likes) },
    { label: '预计分享', value: formatNum(a.shares) },
    { label: '预计评论', value: formatNum(a.comments) },
    { label: '转化率', value: a.conversion_rate + '%' },
    { label: '互动率', value: a.engagement_rate + '%' },
  ]
})

function formatNum(num) {
  if (num >= 10000) return (num / 10000).toFixed(1) + 'w'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'k'
  return num.toString()
}

onMounted(() => {
  if (taskId.value) {
    store.loadResult(taskId.value)
  }
})
</script>