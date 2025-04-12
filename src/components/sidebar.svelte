<script lang="ts">
	export let isSidebarOpen: boolean = false;
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
		dispatch('sidebarToggle', isSidebarOpen);
	}

	function handleKeydown(event: KeyboardEvent, action: () => void) {
		if (event.key === 'Enter' || event.key === ' ') {
			action();
		}
	}
</script>

<div class="relative h-screen overflow-hidden">
	<div
		class={`fixed top-0 left-0 z-40 h-full bg-[#333333] shadow-2xl transition-transform duration-300 ease-in-out ${
			isSidebarOpen ? 'translate-x-0' : '-translate-x-full'
		}`}
		style="width: 280px;"
	>
		<div class="relative h-full p-4 text-white">
			<div
				role="button"
				tabindex="0"
				class="absolute top-4 right-4 cursor-pointer"
				on:click={() => console.log('Edit icon clicked')}
				on:keydown={(event) => handleKeydown(event, () => console.log('Edit icon clicked'))}
				aria-label="Edit"
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 576 512"
					><path
						fill="currentColor"
						d="m402.3 344.9l32-32c5-5 13.7-1.5 13.7 5.7V464c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V112c0-26.5 21.5-48 48-48h273.5c7.1 0 10.7 8.6 5.7 13.7l-32 32c-1.5 1.5-3.5 2.3-5.7 2.3H48v352h352V350.5c0-2.1.8-4.1 2.3-5.6zm156.6-201.8L296.3 405.7l-90.4 10c-26.2 2.9-48.5-19.2-45.6-45.6l10-90.4L432.9 17.1c22.9-22.9 59.9-22.9 82.7 0l43.2 43.2c22.9 22.9 22.9 60 .1 82.8zM460.1 174L402 115.9L216.2 301.8l-7.3 65.3l65.3-7.3L460.1 174zm64.8-79.7l-43.2-43.2c-4.1-4.1-10.8-4.1-14.8 0L436 82l58.1 58.1l30.9-30.9c4-4.2 4-10.8-.1-14.9z"
					/></svg
				>
			</div>
			<div class="mt-16 space-y-4">
				<p>Sidebar Menu Item 1</p>
				<p>Sidebar Menu Item 2</p>
				<p>Sidebar Menu Item 3</p>
			</div>
		</div>
	</div>

	<div
		role="button"
		tabindex="0"
		class="fixed top-4 z-50 cursor-pointer text-white transition-transform duration-300 ease-in-out"
		on:click={toggleSidebar}
		on:keydown={(event) => handleKeydown(event, toggleSidebar)}
		aria-label={isSidebarOpen ? 'Close sidebar' : 'Open sidebar'}
		style={`transform: translateX(${isSidebarOpen ? '300px' : '16px'});`}
	>
		<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24">
			<g fill="none" stroke="currentColor" stroke-width="2">
				<rect
					width="20"
					height="18"
					x="2"
					y="3"
					stroke-linecap="round"
					stroke-linejoin="round"
					rx="2"
				/>
				<path d="M9 3v18" />
			</g>
		</svg>
	</div>
</div>

<style>
	@media (max-width: 768px) {
		div[role='button'] {
			transform: translateX(var(--btn-pos, 16px)) !important;
		}
	}
</style>
