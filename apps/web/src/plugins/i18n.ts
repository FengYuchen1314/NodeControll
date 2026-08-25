import { createI18n } from 'vue-i18n'

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en',
  messages: {
    'zh-CN': {
      app: {
        name: 'NodeControll',
        stage: '工程骨架',
      },
      nav: {
        dashboard: '总览',
        system: '系统',
      },
    },
    en: {
      app: {
        name: 'NodeControll',
        stage: 'Engineering skeleton',
      },
      nav: {
        dashboard: 'Dashboard',
        system: 'System',
      },
    },
  },
})

