<script>
	import Sidebar from '../../components/sidebar.svelte';
</script>

<div class="flex h-full">
	<Sidebar />
	<div class="min-h-0 flex-1">
		<slot />
	</div>
</div>
