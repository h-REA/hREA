import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { viteStaticCopy } from 'vite-plugin-static-copy';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    viteStaticCopy({
      targets: [
        {
          src: 'public/**/*', // Copy all files from the public directory
          dest: '.' // Copy to the root of the output directory
        }
      ]
    })
  ],
  optimizeDeps: { include: ['golden-layout', 'svelte-golden-layout'], },
});