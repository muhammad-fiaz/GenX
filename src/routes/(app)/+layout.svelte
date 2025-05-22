<script>
	import Sidebar from '../../components/sidebar.svelte';
</script>

<div class="flex h-full ">

	<Sidebar />
	<div class="flex-1 min-h-0">
		<slot />
	</div>
</div>