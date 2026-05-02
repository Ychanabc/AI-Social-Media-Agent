<template>
  <div class="admin-page">
    <div class="page-header">
      <h1>⚙️ 管理后台</h1>
      <p>系统配置与用户管理</p>
    </div>

    <el-tabs v-model="activeTab">
      <el-tab-pane label="AI 配置" name="ai">
        <el-card style="margin-bottom: 24px">
          <template #header><span style="font-weight:600">🧠 AI API 配置</span></template>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="Provider">{{ config?.ai?.provider || '-' }}</el-descriptions-item>
            <el-descriptions-item label="Model">{{ config?.ai?.model || '-' }}</el-descriptions-item>
            <el-descriptions-item label="Base URL">{{ config?.ai?.base_url || '-' }}</el-descriptions-item>
            <el-descriptions-item label="API Key">
              <el-tag :type="config?.ai?.api_key_set ? 'success' : 'danger'" size="small">
                {{ config?.ai?.api_key_set ? '已配置' : '未配置' }}
              </el-tag>
            </el-descriptions-item>
          </el-descriptions>
          <el-alert type="info" :closable="false" style="margin-top:16px">
            修改 AI 配置请编辑 backend/.env 文件，修改后需重启后端服务。
          </el-alert>
        </el-card>
        <el-card>
          <template #header><span style="font-weight:600">📱 小红书 API 配置</span></template>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="App ID">
              <el-tag :type="config?.xhs?.app_id_set ? 'success' : 'danger'" size="small">
                {{ config?.xhs?.app_id_set ? '已配置' : '未配置' }}
              </el-tag>
            </el-descriptions-item>
          </el-descriptions>
          <el-alert type="info" :closable="false" style="margin-top:16px">
            小红书开放平台文档：<el-link type="primary" href="https://open.xiaohongshu.com" target="_blank">open.xiaohongshu.com</el-link>
          </el-alert>
        </el-card>
      </el-tab-pane>

      <el-tab-pane label="用户管理" name="users">
        <el-card>
          <template #header>
            <div style="display:flex;justify-content:space-between;align-items:center">
              <span style="font-weight:600">👥 用户列表</span>
              <el-button type="primary" @click="showCreateDialog = true">
                <el-icon><Plus /></el-icon>添加用户
              </el-button>
            </div>
          </template>
          <el-table :data="users" stripe>
            <el-table-column prop="id" label="ID" width="200" />
            <el-table-column prop="username" label="用户名" width="150" />
            <el-table-column prop="display_name" label="显示名称" width="150" />
            <el-table-column prop="role" label="角色" width="120">
              <template #default="{row}">
                <el-tag :type="row.role === 'admin' ? 'danger' : 'info'" size="small">{{ row.role }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="120">
              <template #default="{row}">
                <el-button v-if="row.username !== 'admin'" type="danger" size="small" @click="handleDelete(row.username)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-tab-pane>
    </el-tabs>

    <el-dialog v-model="showCreateDialog" title="添加用户" width="400px">
      <el-form :model="newUser">
        <el-form-item label="用户名"><el-input v-model="newUser.username" /></el-form-item>
        <el-form-item label="密码"><el-input v-model="newUser.password" type="password" /></el-form-item>
        <el-form-item label="显示名称"><el-input v-model="newUser.display_name" /></el-form-item>
        <el-form-item label="角色">
          <el-select v-model="newUser.role" style="width:100%">
            <el-option label="管理员" value="admin" />
            <el-option label="普通用户" value="user" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">取消</el-button>
        <el-button type="primary" @click="handleCreate">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, reactive } from 'vue'
import { useAppStore } from '../store'
import { ElMessage, ElMessageBox } from 'element-plus'

const store = useAppStore()
const activeTab = ref('ai')
const config = ref(null)
const users = ref([])
const showCreateDialog = ref(false)
const newUser = reactive({ username: '', password: '', display_name: '', role: 'user' })

onMounted(async () => {
  try { config.value = await store.getAdminConfig() } catch (e) { console.error(e) }
  try { const r = await store.getAdminUsers(); users.value = r.users } catch (e) { console.error(e) }
})

async function handleDelete(username) {
  await ElMessageBox.confirm(`确认删除用户 ${username}?`, '提示')
  await store.deleteUser(username)
  ElMessage.success('已删除')
  const r = await store.getAdminUsers(); users.value = r.users
}

async function handleCreate() {
  try {
    await store.createUser(newUser)
    ElMessage.success('创建成功')
    showCreateDialog.value = false
    const r = await store.getAdminUsers(); users.value = r.users
  } catch (e) {
    ElMessage.error(e.response?.data || '创建失败')
  }
}
</script>