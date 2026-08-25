<script setup lang="ts">
import { useQuery } from '@tanstack/vue-query'

import { getReadiness, getSystemVersion } from '../api/generated/sdk.gen'
import { formatStartedAt } from '../lib/system'

const version = useQuery({
  queryKey: ['system', 'version'],
  queryFn: async () => {
    const response = await getSystemVersion()
    if (response.error) throw new Error('Unable to load the system version')
    return response.data
  },
})

const readiness = useQuery({
  queryKey: ['system', 'readiness'],
  queryFn: async () => {
    const response = await getReadiness()
    if (response.error) throw new Error('Unable to load readiness')
    return response.data
  },
  refetchInterval: 15_000,
})
</script>

<template>
  <header class="mb-8">
    <p class="text-overline text-primary mb-2">COMPATIBILITY</p>
    <h1 class="text-h4 font-weight-bold mb-2">系统版本</h1>
    <p class="text-body-1 text-medium-emphasis">此页使用 Rust OpenAPI 自动生成的 TypeScript SDK。</p>
  </header>

  <v-card border flat :loading="version.isPending.value">
    <v-card-text v-if="version.data.value" class="pa-6">
      <v-table>
        <tbody>
          <tr>
            <th scope="row">产品</th>
            <td>{{ version.data.value.data.product }}</td>
          </tr>
          <tr>
            <th scope="row">版本</th>
            <td>{{ version.data.value.data.version }}</td>
          </tr>
          <tr>
            <th scope="row">API</th>
            <td>{{ version.data.value.meta.api_version }}</td>
          </tr>
          <tr>
            <th scope="row">启动时间</th>
            <td>{{ formatStartedAt(version.data.value.data.started_at) }}</td>
          </tr>
        </tbody>
      </v-table>
    </v-card-text>
    <v-alert v-else-if="version.isError.value" type="error" variant="tonal" class="ma-6">
      无法读取 Master 版本；请检查 Master 是否启动及反向代理配置。
    </v-alert>
  </v-card>

  <v-card border flat class="mt-6" :loading="readiness.isPending.value">
    <v-card-title>依赖可用性</v-card-title>
    <v-card-text v-if="readiness.data.value">
      <v-list lines="two">
        <v-list-item
          v-for="check in readiness.data.value.checks"
          :key="check.name"
          :title="check.name"
          :subtitle="check.code ?? '检查通过'"
        >
          <template #prepend>
            <v-icon
              :icon="check.status === 'ready' ? 'mdi-check-circle' : 'mdi-alert-circle'"
              :color="check.status === 'ready' ? 'success' : 'error'"
            />
          </template>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>
