<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
	import Input from '../components/input.svelte';
	let message = '';

	function handleInput(event: CustomEvent<string>) {
		message = event.detail;
	}
</script>

<svelte:head>
	<title>{m.title()}</title>
	<meta name="description" content={m.description()} />
</svelte:head>

<div class="flex min-h-screen flex-col items-center justify-center bg-[#1e1e1e] px-4 text-white">
	<h1 class="mb-6 text-center text-2xl font-semibold md:text-3xl">{m.what_can_i_help_with()}</h1>
	<Input {message} on:messageChange={handleInput} />
</div>
