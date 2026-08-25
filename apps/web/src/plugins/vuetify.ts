import 'vuetify/styles'
import { createVuetify } from 'vuetify'

export const vuetify = createVuetify({
  theme: {
    defaultTheme: 'nodecontrollLight',
    themes: {
      nodecontrollLight: {
        dark: false,
        colors: {
          primary: '#3157D5',
          secondary: '#526071',
          background: '#F6F7FB',
          surface: '#FFFFFF',
          error: '#B42318',
          warning: '#B54708',
          success: '#067647',
          info: '#175CD3',
        },
      },
    },
  },
})

