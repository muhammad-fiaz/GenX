<script lang="ts">
	import { page } from '$app/stores';
	import { sidebarItems } from '$lib/sidebarItems';

	$: current = $page.url.pathname;

	let sidebarEl: HTMLElement;

	// Directive to inject raw SVG HTML safely
	function htmlSvg(node: HTMLElement, svg: string) {
		node.innerHTML = svg;
		return {
			update(newSvg: string) {
				node.innerHTML = newSvg;
			}
		};
	}
</script>

<aside
	bind:this={sidebarEl}
	class="flex flex-col items-center overflow-y-auto bg-[#1b1b1b] text-white shadow border-r border-white/10 overflow-hidden"
	style="height: calc(100vh - 2rem);"
>
	<ul class="w-full flex-grow space-y-1 overflow-x-hidden overflow-y-auto py-2">
		{#each sidebarItems as item (item.href)}
			<li>
				<a
					href={item.href}
					aria-label={item.name}
					class="flex h-12 w-full items-center justify-center px-6 text-white hover:bg-gray-500 transition-all duration-200 ease-out transform"
					class:bg-gray-700={current === item.href}
					class:scale-105={current === item.href}
					tabindex="0"
				>
					{@html item.svg}
				</a>

			</li>
		{/each}
	</ul>


	<div class="flex h-16 w-full items-center">
		<button
			aria-label="Logout"
			class="flex h-16 w-full items-center justify-center hover:bg-red-200 focus:text-orange-500 focus:outline-none transition-colors"
		>
			<svg
				class="h-5 w-5 text-red-700"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				viewBox="0 0 24 24"
			>
				<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4" />
				<polyline points="16 17 21 12 16 7" />
				<line x1="21" y1="12" x2="9" y2="12" />
			</svg>
		</button>
	</div>
</aside>
