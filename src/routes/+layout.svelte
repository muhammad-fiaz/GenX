<script lang="ts">
	import '../app.css';
	import Titlebar from '../components/titlebar.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	onMount(() => {
		invoke('show_main_window');
	});

	let { children } = $props();
</script>

<div class="flex min-h-screen flex-col overflow-auto">
	<Titlebar />
	<div class="flex-1">
		{@render children()}
	</div>
</div>
