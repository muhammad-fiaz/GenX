<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';

	const appWindow = getCurrentWindow();
	const isMaximized = writable(false);

	async function updateMaximized() {
		isMaximized.set(await appWindow.isMaximized());
	}

	onMount(() => {
		updateMaximized();

		const unlisten = appWindow.listen('tauri://resize', updateMaximized);

		document.getElementById('titlebar-minimize')?.addEventListener('click', () => appWindow.minimize());

		document.getElementById('titlebar-maximize')?.addEventListener('click', async () => {
			await appWindow.toggleMaximize();
			await updateMaximized();
		});

		document.getElementById('titlebar-close')?.addEventListener('click', () => appWindow.close());

		return () => {
			unlisten.then(fn => fn());
		};
	});

	async function onMouseDown(e: MouseEvent) {
		if (e.button !== 0) return;

		if (e.detail === 2) {
			const maximized = await appWindow.isMaximized();
			if (maximized) {
				await appWindow.unmaximize();
				isMaximized.set(false);
			} else {
				await appWindow.maximize();
				isMaximized.set(true);
			}
		} else {
			appWindow.startDragging();
		}
	}
</script>
<header
	class="flex items-center justify-between select-none px-3 h-8 bg-[#1b1b1b] shadow-inner"
	aria-label="Application Titlebar"
>
	<div
		class="drag-region flex items-center space-x-3 text-sm font-semibold text-white cursor-grab select-none"
		role="button"
		tabindex="0"
		aria-label="Drag window"
		on:mousedown={onMouseDown}
		on:keydown={(e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        appWindow.startDragging();
      }
    }}
	>
		<span>GenX</span>
	</div>

	<div class="flex space-x-1">
		<button
			id="titlebar-minimize"
			type="button"
			class="btn-control w-7 h-7 flex items-center justify-center rounded
        hover:bg-gray-700 focus-visible:bg-gray-700"
			aria-label="Minimize window"
			title="Minimize"
		>
			<svg class="w-3 h-3 text-white" viewBox="0 0 24 24" fill="currentColor">
				<rect x="5" y="11" width="14" height="2"></rect>
			</svg>
		</button>

		<button
			id="titlebar-maximize"
			type="button"
			class="btn-control w-7 h-7 flex items-center justify-center rounded
        hover:bg-gray-700 focus-visible:bg-gray-700"
			aria-label={$isMaximized ? 'Restore window' : 'Maximize window'}
			title={$isMaximized ? 'Restore' : 'Maximize'}
		>
			{#if $isMaximized}
				<svg
					class="w-3 h-3 text-white"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linejoin="round"
					stroke-linecap="round"
				>
					<path d="M4 14h10v6H4z"></path>
					<path d="M14 10V4h6v6z"></path>
				</svg>
			{:else}
				<svg
					class="w-3 h-3 text-white"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linejoin="round"
					stroke-linecap="round"
				>
					<rect x="4" y="4" width="16" height="16"></rect>
				</svg>
			{/if}
		</button>

		<button
			id="titlebar-close"
			type="button"
			class="btn-control w-7 h-7 flex items-center justify-center rounded
        hover:bg-red-600 focus-visible:bg-red-600"
			aria-label="Close window"
			title="Close"
		>
			<svg
				class="w-3 h-3 text-white"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<line x1="6" y1="6" x2="18" y2="18" />
				<line x1="6" y1="18" x2="18" y2="6" />
			</svg>
		</button>
	</div>
</header>

<style>
    :global(.btn-control) {
        -webkit-app-region: no-drag;
    }

    :global(.drag-region) {
        -webkit-app-region: drag;
    }

    button:focus {
        outline: none;
    }
</style>
