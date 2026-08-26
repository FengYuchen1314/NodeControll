import { createI18n } from 'vue-i18n'

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en',
  messages: {
    'zh-CN': {
      app: {
        name: 'NodeControll',
        stage: '自托管控制面',
      },
      nav: {
        dashboard: '总览',
        password: '修改密码',
        security: '账户安全',
        system: '系统',
      },
    },
    en: {
      app: {
        name: 'NodeControll',
        stage: 'Self-hosted control plane',
      },
      nav: {
        dashboard: 'Dashboard',
        password: 'Change password',
        security: 'Account security',
        system: 'System',
      },
    },
  },
})
