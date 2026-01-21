import { defineConfig, Plugin } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'
import fs from 'fs'

function cssProcessingLog(): Plugin {
  return {
    name: 'css-processing-log',
    configureServer(server) {
      console.log('[Vite] CSS processing: enabled')
      console.log('[Vite] CSS theme: default Base UI theme')
      console.log('[Vite] CSS overrides: custom theme overrides')
    },
    buildStart() {
      console.log('[Vite] Build: CSS files will be bundled')
      const themesDir = path.resolve(__dirname, 'themes')
      if (fs.existsSync(themesDir)) {
        const files = fs.readdirSync(themesDir).filter(f => f.endsWith('.css'))
        console.log('[Vite] CSS files found:', files.join(', '))
      }
    }
  }
}

export default defineConfig({
  plugins: [
    tailwindcss(),
    react(),
    cssProcessingLog(),
  ],
  optimizeDeps: {
    include: [
      '@base-ui/react/button',
      '@base-ui/react/checkbox',
      '@base-ui/react/select',
      '@base-ui/react/radio',
      '@base-ui/react/radio-group',
      '@base-ui/react/switch',
      '@base-ui/react/separator',
      '@base-ui/react/number-field',
      '@base-ui/react/dialog',
      '@base-ui/react/menu',
    ],
  },
  esbuild: {
    jsx: 'automatic',
  },
})
