import { defineConfig } from 'vite';

export default defineConfig({
  root: 'web',
  // Relative asset URLs, so the build works from any path.
  base: './',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
});
