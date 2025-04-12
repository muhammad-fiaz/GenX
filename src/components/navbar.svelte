<script lang="ts">
	import { onMount } from 'svelte';
	let dropdownOpen = false;

	function toggleDropdown() {
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.dropdown') && !target.closest('.settings-icon')) {
			dropdownOpen = false;
		}
	}

	onMount(() => {
		document.addEventListener('click', closeDropdown);
		return () => document.removeEventListener('click', closeDropdown);
	});
</script>

<nav class="flex items-center bg-[#1e1e1e] text-white py-4 px-4 w-full h-20 relative">
	<div class="absolute left-1/2 transform -translate-x-1/2 text-4xl font-extrabold tracking-wide leading-none font-[Arial Black]">
		<span class="text-white">GenX</span>
	</div>

	<div class="ml-auto relative settings-icon cursor-pointer" role="button" tabindex="0" on:click={toggleDropdown} on:keydown={(e) => e.key === 'Enter' && toggleDropdown()}>
		<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-label="Settings">
			<path d="M14 3.269C14 2.568 13.432 2 12.731 2H11.27C10.568 2 10 2.568 10 3.269c0 .578-.396 1.074-.935 1.286c-.085.034-.17.07-.253.106c-.531.23-1.162.16-1.572-.249a1.269 1.269 0 0 0-1.794 0L4.412 5.446a1.269 1.269 0 0 0 0 1.794c.41.41.48 1.04.248 1.572a7.946 7.946 0 0 0-.105.253c-.212.539-.708.935-1.286.935C2.568 10 2 10.568 2 11.269v1.462C2 13.432 2.568 14 3.269 14c.578 0 1.074.396 1.286.935c.034.085.07.17.105.253c.231.531.161 1.162-.248 1.572a1.269 1.269 0 0 0 0 1.794l1.034 1.034a1.269 1.269 0 0 0 1.794 0c.41-.41 1.04-.48 1.572-.249c.083.037.168.072.253.106c.539.212.935.708.935 1.286c0 .701.568 1.269 1.269 1.269h1.462c.701 0 1.269-.568 1.269-1.269c0-.578.396-1.074.935-1.287c.085-.033.17-.068.253-.104c.531-.232 1.162-.161 1.571.248a1.269 1.269 0 0 0 1.795 0l1.034-1.034a1.269 1.269 0 0 0 0-1.794c-.41-.41-.48-1.04-.249-1.572c.037-.083.072-.168.106-.253c.212-.539.708-.935 1.286-.935c.701 0 1.269-.568 1.269-1.269V11.27c0-.701-.568-1.269-1.269-1.269c-.578 0-1.074-.396-1.287-.935a7.755 7.755 0 0 0-.105-.253c-.23-.531-.16-1.162.249-1.572a1.269 1.269 0 0 0 0-1.794l-1.034-1.034a1.269 1.269 0 0 0-1.794 0c-.41.41-1.04.48-1.572.249a7.913 7.913 0 0 0-.253-.106C14.396 4.343 14 3.847 14 3.27Z" />
			<path d="M16 12a4 4 0 1 1-8 0a4 4 0 0 1 8 0Z" />
		</svg>

		{#if dropdownOpen}
			<div class="dropdown absolute right-0 mt-2 w-64 bg-[#333333] text-white rounded-lg shadow-lg z-10" role="menu">
				<div class="px-4 py-2 text-lg font-semibold border-b border-gray-500 text-center">
					Settings
				</div>

				<a href="/configuration" class="flex items-center justify-center px-4 py-2 hover:bg-gray-600 rounded-lg">
					<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2" viewBox="0 0 512 512" fill="none" stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="32">
						<path d="M436.67 184.11a27.17 27.17 0 0 1-38.3 0l-22.48-22.49a27.15 27.15 0 0 1 0-38.29l50.89-50.89a.85.85 0 0 0-.26-1.38C393.68 57 351.09 64.15 324.05 91c-25.88 25.69-27.35 64.27-17.87 98a27 27 0 0 1-7.67 27.14l-173 160.76a40.76 40.76 0 1 0 57.57 57.54l162.15-173.3a27 27 0 0 1 26.77-7.7c33.46 8.94 71.49 7.26 97.07-17.94c27.49-27.08 33.42-74.94 20.1-102.33a.85.85 0 0 0-1.36-.22Z" />
					</svg>
					<span class="text-sm">Configuration</span>
				</a>
				<a href="https://github.com/muhammad-fiaz/GenX/issues/new" class="flex items-center justify-center px-4 py-2 hover:bg-gray-600 rounded-lg">
					<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
						<path d="M5 9a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v5a7 7 0 0 1-7 7v0a7 7 0 0 1-7-7V9Zm3-3v-.425c0-.981.384-1.96 1.326-2.238c1.525-.45 3.823-.45 5.348 0C15.616 3.615 16 4.594 16 5.575V6m2.5 1.5L22 4M5.5 7.5L2 4m4 14l-4 3m3-9H1.5m21 0H19m-1 6l4 3m-10-8v8" />
					</svg>
					<span class="text-sm">Issue a Bug?</span>
				</a>
			</div>
		{/if}
	</div>
</nav>