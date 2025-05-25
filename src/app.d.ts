// See https://kit.svelte.dev/docs/types#app for details
import type { User } from '$lib/server/db/schema'; // adjust if your user type is elsewhere

declare global {
	namespace App {
		// interface Error {}
		// interface PageData {}
		interface Locals {
			user?: User | null; // or whatever type your user is
		}
	}
}
