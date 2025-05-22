<script lang="ts">
	import Input from '../../../components/input.svelte';

	let message = '';

	function handleInput(event: CustomEvent<string>) {
		message = event.detail;
	}
</script>

<!-- Chat Page Content -->
<div class="flex h-full w-full flex-1 flex-col">
	<!-- Main chat area (you can add chat messages here in the future) -->
	<div class="flex-1"></div>
	<!-- Fixed Bottom Input -->
	<div class="fixed bottom-3 left-0 w-full">
		<div class="mx-auto max-w-2xl">
			<Input {message} on:messageChange={handleInput} />
		</div>
	</div>
</div>
