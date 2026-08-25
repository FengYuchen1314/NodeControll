<script setup lang="ts">
const foundations: ReadonlyArray<readonly [string, string]> = [
  ['Rust Master', 'Health、version 与 OpenAPI 已建立边界'],
  ['Rust Agent', '独立制品，尚未 enrollment'],
  ['Vue + Vuetify', 'SaaS 壳、路由、主题与 i18n'],
  ['VPS-only gates', '所有编译和测试只在目标服务器执行'],
]
</script>

<template>
  <header class="mb-8">
    <p class="text-overline text-primary mb-2">CONTROL PLANE</p>
    <h1 class="text-h4 font-weight-bold mb-2">重构工程总览</h1>
    <p class="text-body-1 text-medium-emphasis page-lead">
      当前只展示已经落地的工程骨架；业务能力会在对应工作包通过 VPS 验收后逐项开放。
    </p>
  </header>

  <v-row>
    <v-col v-for="([title, detail], index) in foundations" :key="title" cols="12" md="6">
      <v-card border flat class="foundation-card h-100">
        <v-card-item>
          <template #prepend>
            <span class="sequence-badge">{{ String(index + 1).padStart(2, '0') }}</span>
          </template>
          <v-card-title>{{ title }}</v-card-title>
          <v-card-subtitle class="text-wrap">{{ detail }}</v-card-subtitle>
        </v-card-item>
      </v-card>
    </v-col>
  </v-row>
</template>
