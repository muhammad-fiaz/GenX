<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { sidebarItems } from '$lib/sidebarItems';

	$: current = $page.url.pathname;

	let sidebarEl: HTMLElement;

	// Logout handler: call backend to clear cookie, clear localStorage/sessionStorage, and redirect to /signin
	async function logout() {
		try {
			// Call backend to clear the cookie/session
			await fetch('/api/auth/logout', { method: 'POST' });

			// Optionally clear any localStorage or sessionStorage user data (if you store any)
			localStorage.removeItem('user');
			sessionStorage.removeItem('user');
			// (Remove any other relevant keys if needed)

			// Redirect to /signin (or /sign if that's your login page)
			await goto('/signin');
		} catch (e) {
			console.error('Logout failed:', e);
			await goto('/signin');
		}
	}
</script>

<aside
	bind:this={sidebarEl}
	class="flex h-[calc(100vh_-_2rem)] flex-col items-center overflow-hidden overflow-y-auto border-r border-white/10 bg-[#1b1b1b] text-white shadow"
>
	<ul class="w-full flex-grow space-y-1 overflow-x-hidden overflow-y-auto py-2">
		{#each sidebarItems as item (item.href)}
			<li>
				<a
					href={item.href}
					aria-label={item.name}
					class="flex h-12 w-full transform items-center justify-center px-6 text-white transition-all duration-200 ease-out hover:bg-gray-500"
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
			class="flex h-16 w-full items-center justify-center transition-colors hover:bg-red-200 focus:text-orange-500 focus:outline-none"
			on:click={logout}
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
