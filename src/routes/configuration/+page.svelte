<script lang="ts">
	import { writable } from 'svelte/store';
	import { fly } from 'svelte/transition';

	const currentTab = writable<'models' | 'custom' | 'prompt'>('models');
	const isSidebarOpen = writable(false); // For responsive sidebar toggle

	function switchTab(tab: 'models' | 'custom' | 'prompt') {
		currentTab.set(tab);
		isSidebarOpen.set(false); // Close sidebar on mobile after selection
	}
</script>

<div class="flex h-screen bg-[#0f0f0f] text-white">
	<div
		class="fixed inset-y-0 left-0 z-30 w-72 transform space-y-6 bg-[#1a1a1a] p-4 lg:relative lg:inset-y-auto lg:translate-x-0"
		in:fly={{ x: -300 }}
		out:fly={{ x: -300 }}
		style="transition: transform 0.3s ease-in-out;"
		class:translate-x-[-100%]={!$isSidebarOpen}
	>
		<div class="mb-6 flex items-center justify-between">
			<div class="text-xl font-bold">Configurations</div>
			<button
				type="button"
				class="text-gray-400 hover:text-white lg:hidden"
				on:click={() => isSidebarOpen.set(false)}
				aria-label="Close sidebar"
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
					<g fill="currentColor" fill-rule="evenodd" clip-rule="evenodd">
						<path
							d="M5.47 5.47a.75.75 0 0 1 1.06 0l12 12a.75.75 0 1 1-1.06 1.06l-12-12a.75.75 0 0 1 0-1.06"
						/>
						<path
							d="M18.53 5.47a.75.75 0 0 1 0 1.06l-12 12a.75.75 0 0 1-1.06-1.06l12-12a.75.75 0 0 1 1.06 0"
						/>
					</g>
				</svg>
			</button>
		</div>
		<ul class="space-y-4">
			<li>
				<button
					type="button"
					class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-4 py-3 text-left"
					on:click={() => switchTab('models')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('models')}
					class:font-semibold={$currentTab === 'models'}
					class:bg-gray-700={$currentTab === 'models'}
					class:text-white={$currentTab === 'models'}
					class:text-gray-400={$currentTab !== 'models'}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
					</svg>
					Models
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-4 py-3 text-left"
					on:click={() => switchTab('custom')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('custom')}
					class:font-semibold={$currentTab === 'custom'}
					class:bg-gray-700={$currentTab === 'custom'}
					class:text-white={$currentTab === 'custom'}
					class:text-gray-400={$currentTab !== 'custom'}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
					</svg>
					Custom
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-4 py-3 text-left"
					on:click={() => switchTab('prompt')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('prompt')}
					class:font-semibold={$currentTab === 'prompt'}
					class:bg-gray-700={$currentTab === 'prompt'}
					class:text-white={$currentTab === 'prompt'}
					class:text-gray-400={$currentTab !== 'prompt'}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18" />
					</svg>
					Prompt
				</button>
			</li>
		</ul>

		<a
			href="/"
			class="absolute bottom-6 left-6 inline-block rounded-lg bg-gray-700 px-4 py-2 text-white hover:bg-gray-600"
		>
			&larr; Back
		</a>
	</div>

	<!-- Main content -->
	<div class="flex-1 p-6 lg:ml-72">
		<!-- Hamburger Button -->
		<button
			type="button"
			class="mb-4 text-gray-400 hover:text-white lg:hidden"
			on:click={() => isSidebarOpen.set(true)}
			aria-label="Open sidebar"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
				<rect width="18" height="1.5" x="3" y="7.001" fill="currentColor" rx=".75" />
				<rect width="15" height="1.5" x="3" y="11.251" fill="currentColor" rx=".75" />
				<rect width="18" height="1.5" x="3" y="15.499" fill="currentColor" rx=".75" />
			</svg>
		</button>

		<h1 class="mb-6 text-3xl font-bold">
			{#if $currentTab === 'models'}
				Models
			{/if}
			{#if $currentTab === 'custom'}
				Custom
			{/if}
			{#if $currentTab === 'prompt'}
				Prompt
			{/if}
		</h1>

		<div class="rounded-lg bg-[#1a1a1a] p-6">
			{#if $currentTab === 'models'}
				<p class="text-gray-400">
					This is the Models content area. Add your model-related details here.
				</p>
			{/if}
			{#if $currentTab === 'custom'}
				<p class="text-gray-400">
					This is the Custom content area. Add your custom configuration here.
				</p>
			{/if}
			{#if $currentTab === 'prompt'}
				<p class="text-gray-400">
					This is the Prompt content area. Add your prompt-related details here.
				</p>
			{/if}
		</div>
	</div>
</div>
