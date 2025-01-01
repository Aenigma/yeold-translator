import path from 'node:path'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

import oxlintPlugin from 'vite-plugin-oxlint'
import wasm from 'vite-plugin-wasm'


// https://vite.dev/config/
export default defineConfig({
  plugins: [
    oxlintPlugin(),
    react(),
    wasm(),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
})
