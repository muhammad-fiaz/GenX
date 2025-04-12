import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { execSync } from 'child_process';

function getGitVersion() {
	try {
		return execSync('git describe --tags --always').toString().trim();
	} catch {
		return 'dev';
	}
}

const version = getGitVersion().replace(/[^a-zA-Z0-9.-]/g, '_');
const outDir = `genx-build-${version}`;

const config = {
	preprocess: [vitePreprocess(), mdsvex()],
	kit: {
		adapter: adapter({
			pages: outDir,
			assets: outDir,
			fallback: undefined,
			precompress: false,
			strict: true

		}),
		appDir: 'app',
		prerender: {
			entries: ['*']
		}
	},
	extensions: ['.svelte', '.svx']
};

export default config;
