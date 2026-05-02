<template>
  <div class="xhs-page">
    <div class="page-header">
      <h1>📱 小红书数据分析</h1>
      <p>实时追踪笔记表现与互动数据</p>
    </div>

    <!-- 概览卡片 -->
    <el-row :gutter="20" style="margin-bottom:24px">
      <el-col :span="6" v-for="item in overviewCards" :key="item.label">
        <div class="stat-card">
          <div class="stat-icon" :style="{color: item.color}">{{ item.icon }}</div>
          <div class="stat-info">
            <div class="stat-value">{{ formatNum(item.value) }}</div>
            <div class="stat-label">{{ item.label }}</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-bottom:24px">
      <el-col :span="12">
        <el-card>
          <template #header><span style="font-weight:600">📈 每日趋势</span></template>
          <el-table :data="overview?.daily_trend || []" stripe size="small">
            <el-table-column prop="date" label="日期" width="120" />
            <el-table-column prop="views" label="浏览量">
              <template #default="{row}">{{ formatNum(row.views) }}</template>
            </el-table-column>
            <el-table-column prop="likes" label="点赞">
              <template #default="{row}">{{ formatNum(row.likes) }}</template>
            </el-table-column>
            <el-table-column prop="comments" label="评论" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header><span style="font-weight:600">🏆 热门笔记 TOP5</span></template>
          <div v-for="note in (overview?.top_notes || [])" :key="note.note_id" style="padding:12px 0;border-bottom:1px solid #f0f0f0">
            <div style="font-weight:600;margin-bottom:6px">{{ note.title }}</div>
            <div style="display:flex;gap:16px;font-size:13px;color:#909399">
              <span>👁️ {{ formatNum(note.views) }}</span>
              <span>❤️ {{ formatNum(note.likes) }}</span>
              <span>💬 {{ note.comments }}</span>
              <span>⭐ {{ formatNum(note.collections) }}</span>
              <span style="color:#409eff">互动率 {{ note.engagement_rate }}%</span>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 笔记列表 -->
    <el-card>
      <template #header><span style="font-weight:600">📋 笔记列表</span></template>
      <el-table :data="notes" stripe>
        <el-table-column prop="note_id" label="ID" width="100" />
        <el-table-column prop="title" label="标题" min-width="250" show-overflow-tooltip />
        <el-table-column prop="views" label="浏览" width="100">
          <template #default="{row}">{{ formatNum(row.views) }}</template>
        </el-table-column>
        <el-table-column prop="likes" label="点赞" width="100">
          <template #default="{row}">{{ formatNum(row.likes) }}</template>
        </el-table-column>
        <el-table-column prop="comments" label="评论" width="80" />
        <el-table-column prop="shares" label="分享" width="80" />
        <el-table-column prop="collections" label="收藏" width="100">
          <template #default="{row}">{{ formatNum(row.collections) }}</template>
        </el-table-column>
        <el-table-column prop="engagement_rate" label="互动率" width="100">
          <template #default="{row}">
            <el-tag :type="row.engagement_rate > 10 ? 'success' : row.engagement_rate > 5 ? 'warning' : 'info'" size="small">
              {{ row.engagement_rate }}%
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="published_at" label="发布时间" width="170" />
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useAppStore } from '../store'

const store = useAppStore()
const overview = ref(null)
const notes = ref([])

const overviewCards = computed(() => {
  if (!overview.value) return []
  const o = overview.value
  return [
    { icon: '📝', label: '总笔记数', value: o.total_notes, color: '#409eff' },
    { icon: '👁️', label: '总浏览量', value: o.total_views, color: '#67c23a' },
    { icon: '❤️', label: '总点赞', value: o.total_likes, color: '#f56c6c' },
    { icon: '⭐', label: '总收藏', value: o.total_collections, color: '#e6a23c' },
  ]
})

function formatNum(n) {
  if (n >= 10000) return (n / 10000).toFixed(1) + 'w'
  if (n >= 1000) return (n / 1000).toFixed(1) + 'k'
  return String(n)
}

onMounted(async () => {
  try { overview.value = await store.getXhsOverview() } catch (e) { console.error(e) }
  try { const r = await store.getXhsNotes(); notes.value = r.notes } catch (e) { console.error(e) }
})
</script>