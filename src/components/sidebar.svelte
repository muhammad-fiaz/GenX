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
		class={`fixed top-0 left-0 h-full bg-[#333333] shadow-2xl transition-transform duration-300 ease-in-out z-40 ${
			isSidebarOpen ? 'translate-x-0' : '-translate-x-full'
		}`}
		style="width: 280px;"
	>
		<div class="p-4 text-white relative h-full">
			<div
				role="button"
				tabindex="0"
				class="absolute top-4 right-4 cursor-pointer"
				on:click={() => console.log('Edit icon clicked')}
				on:keydown={(event) => handleKeydown(event, () => console.log('Edit icon clicked'))}
				aria-label="Edit"
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="currentColor">
					<g>
						<path d="M21.731 2.269a2.625 2.625 0 0 0-3.712 0l-1.157 1.157l3.712 3.712l1.157-1.157a2.625 2.625 0 0 0 0-3.712Zm-2.218 5.93l-3.712-3.712l-8.4 8.4a5.25 5.25 0 0 0-1.32 2.214l-.8 2.685a.75.75 0 0 0 .933.933l2.685-.8a5.25 5.25 0 0 0 2.214-1.32l8.4-8.4Z"/>
						<path d="M5.25 5.25a3 3 0 0 0-3 3v10.5a3 3 0 0 0 3 3h10.5a3 3 0 0 0 3-3V13.5a.75.75 0 0 0-1.5 0v5.25a1.5 1.5 0 0 1-1.5 1.5H5.25a1.5 1.5 0 0 1-1.5-1.5V8.25a1.5 1.5 0 0 1 1.5-1.5h5.25a.75.75 0 0 0 0-1.5H5.25Z"/>
					</g>
				</svg>
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
		class="fixed top-4 z-50 cursor-pointer transition-transform duration-300 ease-in-out text-white"
		on:click={toggleSidebar}
		on:keydown={(event) => handleKeydown(event, toggleSidebar)}
		aria-label={isSidebarOpen ? 'Close sidebar' : 'Open sidebar'}
		style={`transform: translateX(${isSidebarOpen ? '300px' : '16px'});`}
	>
		<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24">
			<g fill="none" stroke="currentColor" stroke-width="2">
				<rect width="20" height="18" x="2" y="3" stroke-linecap="round" stroke-linejoin="round" rx="2"/>
				<path d="M9 3v18"/>
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
