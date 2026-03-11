import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import dotenv from 'dotenv';

dotenv.config({ path: '../.env' });

const { BACKEND_PORT } = process.env;
const API_BASE_URL = `https://localhost:${BACKEND_PORT ?? 3000}`;

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: API_BASE_URL,
				changeOrigin: true,
				secure: false
			}
		},
		port: 5173
	}
});
