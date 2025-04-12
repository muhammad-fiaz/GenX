<script lang="ts">
	import { m } from '$lib/paraglide/messages.js';
	import Input from '../components/input.svelte';
	import Sidebar from '../components/sidebar.svelte';
	import TopNavbar from '../components/navbar.svelte';

	let message = '';

	function handleInput(event: CustomEvent<string>) {
		message = event.detail;
	}
</script>

<svelte:head>
	<title>{m.title()}</title>
	<meta name="description" content={m.description()} />
</svelte:head>

<!-- Full-screen layout -->
<div class="flex flex-col h-screen">
	<!-- Top Navbar -->
	<TopNavbar />

	<div class="flex flex-1 overflow-hidden">
		<!-- Sidebar -->
		<Sidebar />

		<!-- Main Content -->
		<div class="flex flex-col items-center justify-center flex-1 bg-[#1e1e1e] px-4 text-white">
			<h1 class="text-2xl font-semibold text-center md:text-3xl mb-6">
				{m.what_can_i_help_with()}
			</h1>
			<Input {message} on:messageChange={handleInput} />
		</div>
	</div>
</div>