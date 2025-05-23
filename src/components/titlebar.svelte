<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';

	const appWindow = getCurrentWindow();
	const isMaximized = writable(false);

	// Track open dropdowns for each menu
	let openMenu: string | null = null;

	async function updateMaximized() {
		isMaximized.set(await appWindow.isMaximized());
	}

	onMount(() => {
		updateMaximized();

		const unlisten = appWindow.listen('tauri://resize', updateMaximized);

		document
			.getElementById('titlebar-minimize')
			?.addEventListener('click', () => appWindow.minimize());

		document.getElementById('titlebar-maximize')?.addEventListener('click', async () => {
			await appWindow.toggleMaximize();
			await updateMaximized();
		});

		document.getElementById('titlebar-close')?.addEventListener('click', () => appWindow.close());

		// Close menu on outside click
		const closeMenu = (e: MouseEvent) => {
			if (!(e.target as HTMLElement)?.closest('.titlebar-menu')) {
				openMenu = null;
			}
		};
		window.addEventListener('mousedown', closeMenu);
		return () => {
			unlisten.then((fn) => fn());
			window.removeEventListener('mousedown', closeMenu);
		};
	});

	async function onMouseDown(e: MouseEvent) {
		// Only drag if not interacting with the menu
		if ((e.target as HTMLElement)?.closest('.titlebar-menu')) return;
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

	function toggleMenu(menu: string) {
		openMenu = openMenu === menu ? null : menu;
	}
</script>

<header
	class="flex h-8 items-center justify-between bg-[#1b1b1b] px-3 shadow-inner select-none border-white/10 border-b"
	aria-label="Application Titlebar"
>
	<!-- Logo at the very left -->
	<a href="/" class="flex items-center h-8 w-8 mr-2 shrink-0 select-none -ml-1" tabindex="-1" aria-label="Home">
		<img src="/logo-rounded.png" alt="logo" class="h-4 w-4" draggable="false" />
	</a>
	<!-- Titlebar Menus (draggable background, menus are no-drag) -->
	<div
		class="drag-region flex flex-1 items-center text-sm font-semibold text-white select-none min-w-0 space-x-1"
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
		<!-- File Menu -->
		<div class="titlebar-menu relative z-20">
			<button
				type="button"
				class="btn-control px-2 py-1 rounded hover:bg-gray-700 flex items-center gap-1"
				aria-haspopup="true"
				aria-expanded={openMenu === 'file'}
				aria-label="File Menu"
				on:click|stopPropagation={() => toggleMenu('file')}
			>
				<span>File</span>
				<svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 20 20">
					<path d="M6 8l4 4 4-4"/>
				</svg>
			</button>
			{#if openMenu === 'file'}
				<ul
					class="absolute left-0 mt-1 w-40 rounded-md bg-[#232323] shadow-lg border border-[#444] py-1 text-sm"
					role="menu"
					tabindex="-1"
				>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">New File</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Open File...</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Save</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Save As...</a>
					</li>
				</ul>
			{/if}
		</div>
		<!-- Edit Menu -->
		<div class="titlebar-menu relative z-20">
			<button
				type="button"
				class="btn-control px-2 py-1 rounded hover:bg-gray-700 flex items-center gap-1"
				aria-haspopup="true"
				aria-expanded={openMenu === 'edit'}
				aria-label="Edit Menu"
				on:click|stopPropagation={() => toggleMenu('edit')}
			>
				<span>Edit</span>
				<svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 20 20">
					<path d="M6 8l4 4 4-4"/>
				</svg>
			</button>
			{#if openMenu === 'edit'}
				<ul
					class="absolute left-0 mt-1 w-40 rounded-md bg-[#232323] shadow-lg border border-[#444] py-1 text-sm"
					role="menu"
					tabindex="-1"
				>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Undo</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Redo</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Cut</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Copy</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Paste</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Select All</a>
					</li>
				</ul>
			{/if}
		</div>
		<!-- View Menu -->
		<div class="titlebar-menu relative z-20">
			<button
				type="button"
				class="btn-control px-2 py-1 rounded hover:bg-gray-700 flex items-center gap-1"
				aria-haspopup="true"
				aria-expanded={openMenu === 'view'}
				aria-label="View Menu"
				on:click|stopPropagation={() => toggleMenu('view')}
			>
				<span>View</span>
				<svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 20 20">
					<path d="M6 8l4 4 4-4"/>
				</svg>
			</button>
			{#if openMenu === 'view'}
				<ul
					class="absolute left-0 mt-1 w-40 rounded-md bg-[#232323] shadow-lg border border-[#444] py-1 text-sm"
					role="menu"
					tabindex="-1"
				>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Reload</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Toggle Sidebar</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Toggle Theme</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Zoom In</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Zoom Out</a>
					</li>
					<li>
						<a href="/" class="block px-4 py-2 hover:bg-gray-700 cursor-pointer" role="menuitem" tabindex="0">Reset Zoom</a>
					</li>
				</ul>
			{/if}
		</div>
	</div>

	<div class="ml-auto flex space-x-1">
		<button
			id="titlebar-minimize"
			type="button"
			class="btn-control flex h-7 w-7 items-center justify-center rounded
        hover:bg-gray-700 focus-visible:bg-gray-700"
			aria-label="Minimize window"
			title="Minimize"
		>

			<svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 text-white" viewBox="0 0 24 24"><g fill="none" fill-rule="evenodd"><path d="M24 0v24H0V0h24ZM12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035c-.01-.004-.019-.001-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427c-.002-.01-.009-.017-.017-.018Zm.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093c.012.004.023 0 .029-.008l.004-.014l-.034-.614c-.003-.012-.01-.02-.02-.022Zm-.715.002a.023.023 0 0 0-.027.006l-.006.014l-.034.614c0 .012.007.02.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01l-.184-.092Z"/><path fill="currentColor" d="M2.5 12A1.5 1.5 0 0 1 4 10.5h16a1.5 1.5 0 0 1 0 3H4A1.5 1.5 0 0 1 2.5 12Z"/></g></svg>		</button>

		<button
			id="titlebar-maximize"
			type="button"
			class="btn-control flex h-7 w-7 items-center justify-center rounded
        hover:bg-gray-700 focus-visible:bg-gray-700"
			aria-label={$isMaximized ? 'Restore window' : 'Maximize window'}
			title={$isMaximized ? 'Restore' : 'Maximize'}
		>
			{#if $isMaximized}
				<svg xmlns="http://www.w3.org/2000/svg" 	class="h-3 w-3 text-white" viewBox="0 0 1024 1024"><path fill="currentColor" d="M896 640h-64V512q27 0 45.5-19t18.5-45V192q0-27-18.5-45.5T832 128H448q-27 0-45.5 18.5T384 192v128H256V128q0-53 37.5-90.5T384 0h512q53 0 90.5 37.5T1024 128v384q0 53-37.5 90.5T896 640zM768 512v384q0 53-37.5 90.5T640 1024H128q-53 0-90.5-37.5T0 896V512q0-53 37.5-90.5T128 384h512q53 0 90.5 37.5T768 512zm-192 0H192q-27 0-45.5 18.5T128 576v256q0 26 18.5 45t45.5 19h384q27 0 45.5-19t18.5-45V576q0-27-18.5-45.5T576 512z"/></svg>

			{:else}

				<svg xmlns="http://www.w3.org/2000/svg" 					class="h-3 w-3 text-white"
						 viewBox="0 0 24 24"><path fill="currentColor" d="M6 3h12a3 3 0 0 1 3 3v12a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6a3 3 0 0 1 3-3Zm0 2a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1H6Z"/></svg>
			{/if}
		</button>

		<button
			id="titlebar-close"
			type="button"
			class="btn-control flex h-7 w-7 items-center justify-center rounded
        hover:bg-red-600 focus-visible:bg-red-600"
			aria-label="Close window"
			title="Close"
		>

			<svg xmlns="http://www.w3.org/2000/svg" 				class="h-3 w-3 text-white"
					 viewBox="0 0 20 20"><path fill="currentColor" d="M10 8.586L2.929 1.515L1.515 2.929L8.586 10l-7.071 7.071l1.414 1.414L10 11.414l7.071 7.071l1.414-1.414L11.414 10l7.071-7.071l-1.414-1.414L10 8.586z"/></svg>
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

    .titlebar-menu, .titlebar-menu * {
        -webkit-app-region: no-drag !important;
    }

    button:focus {
        outline: none;
    }
</style>