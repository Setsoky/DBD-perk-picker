import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

export default createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'dark',
        dark: {
          light: {
            colors: {
              primary: '#252526',
              secondary: '#0d730d',
            },
          },
        },
      },
  })


