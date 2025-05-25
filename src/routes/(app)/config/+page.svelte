<script lang="ts">
	import { writable } from 'svelte/store';
	import Provider from '../../../components/provider.svelte';
	import Models from '../../../components/models.svelte';
	import Custom from '../../../components/custom.svelte';
	import Prompt from '../../../components/prompt.svelte';
	import Downloads from '../../../components/downloads.svelte';

	const currentTab = writable<'models' | 'custom' | 'prompt' | 'provider' | 'downloads'>('models');

	function switchTab(tab: 'models' | 'custom' | 'prompt' | 'provider' | 'downloads') {
		currentTab.set(tab);
	}
</script>

<div class="flex h-full bg-[#0f0f0f] text-white">
	<!-- Sidebar: always visible, never overlays -->
	<div
		class="flex w-50 flex-col space-y-6 bg-[#1a1a1a] p-4"
		style="box-shadow: 2px 0 8px 0 rgb(0 0 0 / 0.15);"
	>
		<div class="mb-6 text-xl font-bold">Configurations</div>
		<ul class="space-y-4">
			<li>
				<button
					type="button"
					class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors
						{$currentTab === 'models' ? 'bg-gray-700 font-semibold text-white' : 'text-gray-400'}"
					on:click={() => switchTab('models')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('models')}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 transition-colors"
						viewBox="0 0 32 32"
						class:text-white={$currentTab === 'models'}
						class:text-gray-400={$currentTab !== 'models'}
					>
						<path
							fill="currentColor"
							d="M28.447 16.105L23 13.383V7a1 1 0 0 0-.553-.894l-6-3a1 1 0 0 0-.894 0l-6 3A1 1 0 0 0 9 7v6.382l-5.447 2.723A1 1 0 0 0 3 17v7a1 1 0 0 0 .553.894l6 3a1.001 1.001 0 0 0 .894 0L16 25.119l5.553 2.777a1.001 1.001 0 0 0 .894 0l6-3A1 1 0 0 0 29 24v-7a1 1 0 0 0-.553-.895ZM21 13.383l-4 2v-4.764l4-2Zm-5-8.264L19.764 7L16 8.882L12.236 7Zm-5 3.5l4 2v4.764l-4-2ZM9 25.382l-4-2v-4.764l4 2Zm1-6.5L6.236 17L10 15.118L13.764 17Zm1 1.736l4-2v4.764l-4 2Zm10 4.764l-4-2v-4.764l4 2Zm1-6.5L18.236 17L22 15.118L25.764 17Zm5 4.5l-4 2v-4.764l4-2Z"
						/>
					</svg>
					Models
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors
						{$currentTab === 'provider' ? 'bg-gray-700 font-semibold text-white' : 'text-gray-400'}"
					on:click={() => switchTab('provider')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('provider')}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 transition-colors"
						viewBox="0 0 24 24"
						class:text-white={$currentTab === 'provider'}
						class:text-gray-400={$currentTab !== 'provider'}
					>
						<g fill="none" fill-rule="evenodd">
							<path
								d="M24 0v24H0V0h24ZM12.594 23.258l-.012.002l-.071.035l-.02.004l-.014-.004l-.071-.036c-.01-.003-.019 0-.024.006l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427c-.002-.01-.009-.017-.016-.018Zm.264-.113l-.014.002l-.184.093l-.01.01l-.003.011l.018.43l.005.012l.008.008l.201.092c.012.004.023 0 .029-.008l.004-.014l-.034-.614c-.003-.012-.01-.02-.02-.022Zm-.715.002a.023.023 0 0 0-.027.006l-.006.014l-.034.614c0 .012.007.02.017.024l.015-.002l.201-.093l.01-.008l.003-.011l.018-.43l-.003-.012l-.01-.01l-.184-.092Z"
							/>
							<path
								fill="currentColor"
								d="M16 15c1.306 0 2.418.835 2.83 2H20a1 1 0 1 1 0 2h-1.17a3.001 3.001 0 0 1-5.66 0H4a1 1 0 1 1 0-2h9.17A3.001 3.001 0 0 1 16 15Zm0 2a1 1 0 1 0 0 2a1 1 0 0 0 0-2ZM8 9a3 3 0 0 1 2.762 1.828l.067.172H20a1 1 0 0 1 .117 1.993L20 13h-9.17a3.001 3.001 0 0 1-5.592.172L5.17 13H4a1 1 0 0 1-.117-1.993L4 11h1.17A3.001 3.001 0 0 1 8 9Zm0 2a1 1 0 1 0 0 2a1 1 0 0 0 0-2Zm8-8c1.306 0 2.418.835 2.83 2H20a1 1 0 1 1 0 2h-1.17a3.001 3.001 0 0 1-5.66 0H4a1 1 0 0 1 0-2h9.17A3.001 3.001 0 0 1 16 3Zm0 2a1 1 0 1 0 0 2a1 1 0 0 0 0-2Z"
							/>
						</g>
					</svg>
					Provider
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors
						{$currentTab === 'prompt' ? 'bg-gray-700 font-semibold text-white' : 'text-gray-400'}"
					on:click={() => switchTab('prompt')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('prompt')}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 transition-colors"
						viewBox="0 0 24 24"
						class:text-white={$currentTab === 'prompt'}
						class:text-gray-400={$currentTab !== 'prompt'}
					>
						<path
							fill="currentColor"
							d="M19.75 6.5a.75.75 0 0 1-.75-.75V5h-.75a.75.75 0 0 1 0-1.5H19v-.75a.75.75 0 0 1 1.5 0v.75h.75a.75.75 0 0 1 0 1.5h-.75v.75a.75.75 0 0 1-.75.75M8.799 3.728a.34.34 0 0 1 .1-.18c.046-.04.082-.048.102-.048s.056.007.102.049a.34.34 0 0 1 .1.178a6.57 6.57 0 0 0 1.767 3.304a6.55 6.55 0 0 0 3.303 1.767c.07.014.135.053.178.1c.042.046.049.082.049.102s-.007.056-.049.102a.34.34 0 0 1-.178.1a6.56 6.56 0 0 0-3.303 1.768a6.56 6.56 0 0 0-1.766 3.302a.34.34 0 0 1-.101.18c-.046.04-.082.048-.102.048s-.056-.007-.102-.049a.34.34 0 0 1-.1-.178a6.57 6.57 0 0 0-1.767-3.304a6.55 6.55 0 0 0-3.303-1.767a.34.34 0 0 1-.179-.1c-.04-.047-.048-.082-.049-.102c0-.02.008-.056.049-.101a.34.34 0 0 1 .18-.102a6.56 6.56 0 0 0 3.302-1.766a6.56 6.56 0 0 0 1.767-3.303M9 2c-.883 0-1.52.695-1.67 1.422A5.06 5.06 0 0 1 5.97 5.97a5.06 5.06 0 0 1-2.546 1.36c-.728.15-1.424.788-1.423 1.673c.002.882.697 1.517 1.423 1.668a5.05 5.05 0 0 1 2.547 1.358c.87.871 1.22 1.88 1.359 2.549C7.48 15.305 8.118 16 9 16c.885 0 1.521-.695 1.672-1.423a5.06 5.06 0 0 1 1.358-2.546a5.06 5.06 0 0 1 2.548-1.36c.727-.15 1.422-.787 1.422-1.67c0-.885-.695-1.521-1.423-1.672a5.05 5.05 0 0 1-2.546-1.359a5.06 5.06 0 0 1-1.359-2.548C10.521 2.695 9.885 2 9.002 2M9.5 16.954v1.796c0 .966.784 1.75 1.75 1.75h7.5a1.75 1.75 0 0 0 1.75-1.75v-7.5a1.75 1.75 0 0 0-1.75-1.75h-1.796a2.7 2.7 0 0 0-.15-1.5h1.946A3.25 3.25 0 0 1 22 11.25v7.5A3.25 3.25 0 0 1 18.75 22h-7.5A3.25 3.25 0 0 1 8 18.75v-1.946a2.7 2.7 0 0 0 1.5.15M12 13.75a.75.75 0 0 1 .75-.75h5.5a.75.75 0 0 1 0 1.5h-5.5a.75.75 0 0 1-.75-.75m.75 2.25a.75.75 0 0 0 0 1.5h3.5a.75.75 0 0 0 0-1.5zm-10 3a.75.75 0 0 0 0 1.5h.75v.75a.75.75 0 0 0 1.5 0v-.75h.75a.75.75 0 0 0 0-1.5H5v-.75a.75.75 0 0 0-1.5 0V19z"
						/>
					</svg>
					Prompt
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors
						{$currentTab === 'downloads' ? 'bg-gray-700 font-semibold text-white' : 'text-gray-400'}"
					on:click={() => switchTab('downloads')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('downloads')}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 transition-colors"
						viewBox="0 0 15 15"
						class:text-white={$currentTab === 'downloads'}
						class:text-gray-400={$currentTab !== 'downloads'}
					>
						<path
							fill="currentColor"
							fill-rule="evenodd"
							d="M7.5 1.05a.45.45 0 0 1 .45.45v6.914l2.232-2.232a.45.45 0 1 1 .636.636l-3 3a.45.45 0 0 1-.636 0l-3-3a.45.45 0 1 1 .636-.636L7.05 8.414V1.5a.45.45 0 0 1 .45-.45ZM2.5 10a.5.5 0 0 1 .5.5V12c0 .554.446 1 .996 1h7.005A.999.999 0 0 0 12 12v-1.5a.5.5 0 0 1 1 0V12a2 2 0 0 1-1.999 2H3.996A1.997 1.997 0 0 1 2 12v-1.5a.5.5 0 0 1 .5-.5Z"
							clip-rule="evenodd"
						/>
					</svg>
					Downloads
				</button>
			</li>
			<li>
				<button
					type="button"
					class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors
						{$currentTab === 'custom' ? 'bg-gray-700 font-semibold text-white' : 'text-gray-400'}"
					on:click={() => switchTab('custom')}
					on:keydown={(e) => e.key === 'Enter' && switchTab('custom')}
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-6 w-6 transition-colors"
						viewBox="0 0 24 24"
						class:text-white={$currentTab === 'custom'}
						class:text-gray-400={$currentTab !== 'custom'}
					>
						<path
							fill="none"
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-miterlimit="10"
							stroke-width="1.5"
							d="M14 17h6m-3 3v-6M5.6 4h2.8A1.6 1.6 0 0 1 10 5.6v2.8A1.6 1.6 0 0 1 8.4 10H5.6A1.6 1.6 0 0 1 4 8.4V5.6A1.6 1.6 0 0 1 5.6 4m0 10h2.8a1.6 1.6 0 0 1 1.6 1.6v2.8A1.6 1.6 0 0 1 8.4 20H5.6A1.6 1.6 0 0 1 4 18.4v-2.8A1.6 1.6 0 0 1 5.6 14m10-10h2.8A1.6 1.6 0 0 1 20 5.6v2.8a1.6 1.6 0 0 1-1.6 1.6h-2.8A1.6 1.6 0 0 1 14 8.4V5.6A1.6 1.6 0 0 1 15.6 4"
						/>
					</svg>
					Custom
				</button>
			</li>
		</ul>
	</div>

	<!-- Main Content -->
	<div class="flex-1 p-6">
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
			{#if $currentTab === 'downloads'}
				Downloads
			{/if}
			{#if $currentTab === 'provider'}
				Provider
			{/if}
		</h1>
		<div class="rounded-lg bg-[#1a1a1a] p-6">
			{#if $currentTab === 'models'}
				<Models />
			{:else if $currentTab === 'provider'}
				<Provider />
			{:else if $currentTab === 'custom'}
				<Custom />
			{:else if $currentTab === 'downloads'}
				<Downloads />
			{:else if $currentTab === 'prompt'}
				<Prompt />
			{/if}
		</div>
	</div>
</div>
