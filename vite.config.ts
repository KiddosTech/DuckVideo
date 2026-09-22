import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'node:path';

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      vue: 'vue/dist/vue.esm-bundler.js'
    }
  },
  server: {
    watch: {
      ignored: ['**/src-tauri/target/**', '**/src-tauri/gen/**']
    }
  },
  base: './',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    rollupOptions: {
      input: {
        index: resolve(__dirname, 'index.html'),
        download: resolve(__dirname, 'download.html'),
        compress: resolve(__dirname, 'compress.html'),
        logs: resolve(__dirname, 'logs.html'),
        about: resolve(__dirname, 'about.html'),
        settings: resolve(__dirname, 'settings.html')
      }
    }
  }
});
