<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";

	interface OllamaConfig {
		id?: string; // optional if not in OllamaConfig
		name: string;
		version: string;
		description: string;
		type?: string;
		configPath?: string;
		enabled?: boolean;
		host: string;
		endpoints: never[];
		settings: never[];
	}

	let integrations: OllamaConfig[] = [];

	onMount(async () => {
		try {
			integrations = await invoke<OllamaConfig[]>("get_enabled_integrations");
			console.log("Integrations:", integrations);
		} catch (e) {
			console.error("Failed to get integrations:", e);
		}
	});
</script>
