<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";

	// TypeScript interfaces matching Rust structs
	interface MetaIntegration {
		id: string;
		name: string;
		version: string;
		description: string;
		type: string;
		configPath: string;
		enabled: boolean;
	}

	interface Endpoint {
		key: string;
		path: string;
		method: string;
		description: string;
	}

	interface Setting {
		key: string;
		label: string;
		type: string;
		default: never;
		min?: number;
		max?: number;
		step?: number;
		options?: string[];
		description: string;
	}

	interface IntegrationConfig {
		name: string;
		version: string;
		description: string;
		host: string;
		endpoints: Endpoint[];
		settings: Setting[];
	}

	let metaIntegrations: MetaIntegration[] = [];
	let integrationConfigs: IntegrationConfig[] = [];

	onMount(async () => {
		try {
			// First get the meta information about all integrations
			metaIntegrations = await invoke<MetaIntegration[]>('get_meta_integrations');
			console.log("Meta Integrations:", metaIntegrations);

			// Then get the detailed configs for enabled integrations
			integrationConfigs = await invoke<IntegrationConfig[]>('get_enabled_integrations_configs');
			console.log("Integration Configs:", integrationConfigs);

			// Log all settings from all integrations
			console.log("All Integration Settings:");
			integrationConfigs.forEach((config, index) => {
				console.log(`\nSettings for ${config.name} (Integration ${index + 1}):`);
				config.settings.forEach(setting => {
					console.log(`- ${setting.key}: ${setting.default} (${setting.type})`);
				});
			});

		} catch (e) {
			console.error("Failed to load integrations:", e);
		}
	});
</script>
