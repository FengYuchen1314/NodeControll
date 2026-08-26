import { VueQueryPlugin } from '@tanstack/vue-query'
import { createApp } from 'vue'

import App from './App.vue'
import { i18n } from './plugins/i18n'
import { vuetify } from './plugins/vuetify'
import { router } from './router'
import { pinia } from './stores'
import './styles/main.scss'

createApp(App)
  .use(pinia)
  .use(VueQueryPlugin)
  .use(i18n)
  .use(router)
  .use(vuetify)
  .mount('#app')
