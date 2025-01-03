import path from 'node:path'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

import oxlintPlugin from 'vite-plugin-oxlint'
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from 'vite-plugin-wasm'


// https://vite.dev/config/
export default defineConfig({
  base: '',
  plugins: [
    oxlintPlugin(),
    react(),
    topLevelAwait(),
    wasm(),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    target: 'esnext',
  }
})
