import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { type Plugin, defineConfig } from 'vite';

import { handle } from './server/fetch-page';

const api = (): Plugin => ({
  name: 'api',
  configureServer(server) {
    server.middlewares.use('/api/fetch', async (request, response) => {
      const url = (request as { originalUrl?: string }).originalUrl ?? '/';

      const result = await handle(
        new Request(new URL(url, 'http://localhost'))
      );

      response.statusCode = result.status;

      result.headers.forEach((value, key) => response.setHeader(key, value));

      response.end(await result.text());
    });
  },
});

export default defineConfig({
  css: { devSourcemap: true },
  plugins: [react(), tailwindcss(), api()],
  server: { fs: { allow: ['..'] } },
  resolve: {
    alias: {
      '@': path.resolve(import.meta.dirname, './src'),
    },
  },
});
