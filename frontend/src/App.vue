<template>
  <div id="app">
    <aside class="sidebar" v-if="route.path !== '/login'">
      <div class="logo">
        <h2>🤖 AI 社媒系统</h2>
        <p>Multi-Agent Pipeline</p>
      </div>
      <ul class="sidebar-nav">
        <li>
          <router-link to="/">
            <el-icon><DataBoard /></el-icon>
            <span>数据总览</span>
          </router-link>
        </li>
        <li>
          <router-link to="/generate">
            <el-icon><MagicStick /></el-icon>
            <span>内容生成</span>
          </router-link>
        </li>
        <li>
          <router-link to="/tasks">
            <el-icon><List /></el-icon>
            <span>任务列表</span>
          </router-link>
        </li>
        <li>
          <router-link to="/xhs">
            <el-icon><TrendCharts /></el-icon>
            <span>小红书分析</span>
          </router-link>
        </li>
        <li v-if="store.isAdmin">
          <router-link to="/admin">
            <el-icon><Setting /></el-icon>
            <span>管理后台</span>
          </router-link>
        </li>
      </ul>
      <div class="user-info" v-if="store.currentUser">
        <div class="user-name">{{ store.currentUser.display_name }}</div>
        <div class="user-role">
          <el-tag :type="store.isAdmin ? 'danger' : 'info'" size="small">{{ store.currentUser.role }}</el-tag>
        </div>
        <el-button text size="small" @click="handleLogout" style="margin-top:8px;color:#f56c6c">退出登录</el-button>
      </div>
    </aside>
    <main class="main-content" :style="route.path === '/login' ? 'margin-left:0;padding:0' : ''">
      <router-view />
    </main>
  </div>
</template>

<script setup>
import { useRoute, useRouter } from 'vue-router'
import { useAppStore } from './store'

const route = useRoute()
const router = useRouter()
const store = useAppStore()

function handleLogout() {
  store.logout()
  router.push('/login')
}
</script>

<style scoped>
.user-info {
  position: absolute;
  bottom: 20px;
  left: 20px;
  right: 20px;
  padding: 12px;
  background: rgba(255,255,255,0.05);
  border-radius: 8px;
  text-align: center;
}
.user-name {
  font-size: 14px;
  font-weight: 600;
  color: #e0e0e0;
}
.user-role {
  margin-top: 4px;
}
</style>