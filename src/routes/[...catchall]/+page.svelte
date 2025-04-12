<script lang="ts">
	import { goto } from '$app/navigation';

	function goBack() {
		goto('/');
	}
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-[#1e1e1e] text-white text-center px-4">
	<h1 class="font-extrabold leading-none" style="font-size: 12rem; line-height: 1;">404</h1>
	<p class="text-xl mb-6">Oops! The page you are looking for doesn't exist.</p>
	<button
		on:click={goBack}
		class="bg-white text-black px-6 py-2 rounded-lg font-semibold hover:bg-gray-200 transition">
		Go Back
	</button>
</div>