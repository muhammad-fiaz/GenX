<script lang="ts">
	import { goto } from '$app/navigation';

	function goBack() {
		goto('/');
	}
</script>

<div
	class="flex min-h-screen flex-col items-center justify-center bg-[#1e1e1e] px-4 text-center text-white"
>
	<h1 class="leading-none font-extrabold" style="font-size: 12rem; line-height: 1;">404</h1>
	<p class="mb-6 text-xl">Oops! The page you are looking for doesn't exist.</p>
	<button
		on:click={goBack}
		class="rounded-lg bg-white px-6 py-2 font-semibold text-black transition hover:bg-gray-200"
	>
		Go Back
	</button>
</div>
