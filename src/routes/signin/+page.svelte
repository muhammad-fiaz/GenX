<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let loading = false;
	let checking = false;
	let success = false;
	let error: string | null = null;
	let cancelled = false;

	onMount(async () => {
		checking = true;
		loading = false;
		cancelled = false;
		const resp = await fetch('/api/auth/session');
		const { loggedIn } = await resp.json();
		if (loggedIn) {
			goto('/chat');
			return;
		}
		checking = false;
	});

	async function loginWithGitHub() {
		loading = true;
		checking = false;
		error = null;
		success = false;
		cancelled = false;
		try {
			const user = await invoke('github_login');
			if (cancelled) {
				loading = false;
				return;
			}
			if (user) {
				const resp = await fetch('/api/auth/github', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify(user)
				});
				if (cancelled) {
					loading = false;
					return;
				}
				if (resp.ok) {
					success = true;
					setTimeout(() => {
						goto('/chat');
					}, 1000);
				} else {
					const msg = await resp.text();
					error = 'Failed to store user in cookie: ' + msg;
				}
			} else {
				error = 'GitHub login failed: unknown error';
			}
		} catch (e) {
			if (!cancelled) {
				error = 'GitHub login failed: ' + e;
			}
		} finally {
			if (!success) {
				loading = false;
			}
		}
	}

	function cancel() {
		cancelled = true;
		loading = false;
	}
</script>

<div class="h-[calc(100vh_-_2rem)] flex flex-col items-center justify-center bg-transparent">
	<div class="rounded-2xl px-8 py-8 w-full max-w-sm flex flex-col items-center">
		{#if checking}
			<div class="flex flex-col items-center w-full">
				<svg class="animate-spin h-16 w-16 text-gray-800 mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
					<path class="opacity-75" fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
				</svg>
				<div class="text-lg text-center font-medium text-gray-800">Checking session...</div>
			</div>
		{:else}
			{#if loading}
				<div class="flex flex-col items-center w-full mb-5">
					<svg class="animate-spin h-16 w-16 text-gray-800 mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
						<path class="opacity-75" fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
					</svg>
					<div class="text-lg text-center font-medium text-gray-800 mb-2">Signing in with GitHub...</div>
					<button class="mt-2 px-5 py-1.5 bg-transparent border border-gray-800 text-gray-800 rounded-lg hover:bg-gray-800 hover:text-white transition" on:click={cancel}>Cancel</button>
				</div>
			{/if}
			{#if !loading}
				<button
					class="w-full bg-gray-800 text-white px-4 py-3 rounded-lg font-semibold hover:bg-black transition mb-2"
					on:click={loginWithGitHub}
					disabled={loading || checking}
				>
					Sign in with GitHub
				</button>
			{/if}
			{#if success}
				<div class="text-green-600 mt-4 text-center text-base">Successfully signed in! Redirecting...</div>
			{/if}
			{#if error}
				<div class="text-red-600 mt-4 text-center text-base">{error}</div>
			{/if}
		{/if}
	</div>
</div>