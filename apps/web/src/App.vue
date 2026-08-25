<script setup lang="ts">
import { useDisplay } from 'vuetify'

const { mobile } = useDisplay()
</script>

<template>
  <v-app>
    <v-navigation-drawer :rail="!mobile" :permanent="!mobile">
      <div class="brand-lockup px-4 py-5">
        <span class="brand-mark" aria-hidden="true">N</span>
        <div v-if="mobile" class="ml-3">
          <div class="text-subtitle-1 font-weight-bold">{{ $t('app.name') }}</div>
          <div class="text-caption text-medium-emphasis">{{ $t('app.stage') }}</div>
        </div>
      </div>

      <v-divider />
      <v-list nav density="comfortable" aria-label="Primary navigation">
        <v-list-item to="/" :title="$t('nav.dashboard')" value="dashboard" />
        <v-list-item to="/setup" title="初始化" value="setup" />
        <v-list-item to="/system" :title="$t('nav.system')" value="system" />
      </v-list>
    </v-navigation-drawer>

    <v-app-bar flat border="b">
      <v-app-bar-title>{{ $t('app.name') }}</v-app-bar-title>
      <template #append>
        <v-chip color="info" variant="tonal" size="small">P5 · Foundation</v-chip>
      </template>
    </v-app-bar>

    <v-main>
      <v-container class="page-container py-8">
        <router-view />
      </v-container>
    </v-main>
  </v-app>
</template>
