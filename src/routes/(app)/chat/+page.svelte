<script lang="ts">
	import Input from '../../../components/input.svelte';

	let message = '';

	function handleInput(event: CustomEvent<string>) {
		message = event.detail;
	}
</script>

<!-- Chat Page Content -->
<div class="flex h-full w-full flex-1 flex-col min-w-0 min-h-0 overflow-hidden">
	<!-- Main chat area (expandable, future messages go here) -->
	<div class="flex-1 min-h-0 min-w-0 overflow-auto">
		<!-- Chat messages will go here -->
	</div>
	<!-- Fixed Bottom Input -->
	<div class="sticky bottom-0 left-0 w-full z-10">
		<div class="mx-auto max-w-2xl px-2 py-2">
			<Input {message} on:messageChange={handleInput} />
		</div>
	</div>
</div>