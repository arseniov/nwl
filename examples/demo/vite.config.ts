import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  css: {
    postcss: './postcss.config.js',
  },
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
