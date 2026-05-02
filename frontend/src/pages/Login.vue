<template>
  <div class="login-container">
    <div class="login-box">
      <div class="login-header">
        <h1>🤖 AI 社媒自动化系统</h1>
        <p>Multi-Agent Pipeline</p>
      </div>
      <el-form :model="form" @submit.prevent="handleLogin">
        <el-form-item>
          <el-input v-model="form.username" placeholder="用户名" prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item>
          <el-input v-model="form.password" type="password" placeholder="密码" prefix-icon="Lock" size="large" show-password />
        </el-form-item>
        <el-button type="primary" size="large" style="width: 100%" :loading="loading" @click="handleLogin">
          登录
        </el-button>
      </el-form>
      <div class="login-hint">
        <p>管理员：admin / admin123</p>
        <p>用户：user / user123</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAppStore } from '../store'
import { ElMessage } from 'element-plus'

const router = useRouter()
const store = useAppStore()
const loading = ref(false)
const form = reactive({ username: '', password: '' })

async function handleLogin() {
  if (!form.username || !form.password) {
    ElMessage.warning('请输入用户名和密码')
    return
  }
  loading.value = true
  try {
    await store.login(form.username, form.password)
    ElMessage.success('登录成功')
    router.push('/')
  } catch (err) {
    ElMessage.error(err.response?.data || '登录失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
.login-box {
  background: #fff;
  border-radius: 16px;
  padding: 40px;
  width: 400px;
  box-shadow: 0 20px 60px rgba(0,0,0,0.2);
}
.login-header {
  text-align: center;
  margin-bottom: 30px;
}
.login-header h1 {
  font-size: 22px;
  margin-bottom: 8px;
}
.login-header p {
  color: #909399;
  font-size: 13px;
}
.login-hint {
  margin-top: 16px;
  text-align: center;
  font-size: 12px;
  color: #c0c4cc;
}
.login-hint p {
  margin: 4px 0;
}
</style>