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

<div
	class="flex min-h-screen w-full flex-col items-center justify-center bg-gradient-to-br from-gray-900 via-slate-800 to-gray-900 px-4 text-center font-sans text-white"
>
	<h1 class="mb-4 text-4xl font-bold tracking-wide sm:text-5xl lg:text-6xl">GenX</h1>
	<p class="text-base text-gray-300 sm:text-lg md:text-xl">Setting things up. Please wait...</p>
	<div
		class="mt-6 h-10 w-10 animate-spin rounded-full border-4 border-white/20 border-t-cyan-400"
	></div>
</div>
