<script lang="ts">
	import Input from '../../../components/input.svelte';

	let message = '';

	function handleInput(event: CustomEvent<string>) {
		message = event.detail;
	}
</script>

<!-- Chat Page Content -->
<div class="flex h-full min-h-0 w-full min-w-0 flex-1 flex-col overflow-hidden">
	<!-- Main chat area (expandable, future messages go here) -->
	<div class="min-h-0 min-w-0 flex-1 overflow-auto">
		<!-- Chat messages will go here -->
	</div>
	<!-- Fixed Bottom Input -->
	<div class="sticky bottom-0 left-0 z-10 w-full">
		<div class="mx-auto max-w-2xl px-2 py-2">
			<Input {message} on:messageChange={handleInput} />
		</div>
	</div>
</div>
