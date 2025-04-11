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
    <meta name="description" content="{m.description()}" />
</svelte:head>

<div class="min-h-screen bg-[#1e1e1e] text-white flex flex-col items-center justify-center px-4">
    <h1 class="text-2xl md:text-3xl font-semibold mb-6 text-center">{m.what_can_i_help_with()}</h1>
    <Input {message} on:messageChange={handleInput} />
</div>
