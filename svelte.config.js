import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { execSync } from 'child_process';

function getGitTag() {
	try {
		const tag = execSync('git describe --tags --abbrev=0').toString().trim();
		return tag || 'latest';
	} catch {
		return 'latest';
	}
}

const gitTag = getGitTag().replace(/[^a-zA-Z0-9.-]/g, '_');
const isProd = process.env.NODE_ENV === 'production' || process.env.MODE === 'production';
const buildType = isProd ? 'prod' : 'dev';
const outDir = `genx-build-${gitTag}-${buildType}`;

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
