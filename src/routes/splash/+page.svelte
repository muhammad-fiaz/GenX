<script lang="ts">
	import { onMount } from 'svelte';

	async function sleep(seconds: number) {
		return new Promise((res) => setTimeout(res, seconds * 1000));
	}

	onMount(async () => {
		console.log('Simulating frontend setup...');
		await sleep(3);
		console.log('Frontend setup done!');

	});
</script>

<div class="flex flex-col items-center justify-center min-h-screen w-full px-4 text-center bg-gradient-to-br from-gray-900 via-slate-800 to-gray-900 text-white font-sans">
	<h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-wide mb-4">GenX</h1>
	<p class="text-base sm:text-lg md:text-xl text-gray-300">Setting things up. Please wait...</p>
	<div class="mt-6 w-10 h-10 border-4 border-white/20 border-t-cyan-400 rounded-full animate-spin"></div>
</div>
