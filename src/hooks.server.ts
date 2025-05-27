import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { paraglideMiddleware } from '$lib/paraglide/server';

export const handle: Handle = async ({ event, resolve }) => {
	const url = new URL(event.request.url);

	// Skip paraglideMiddleware for .well-known and other static paths
	if (url.pathname.startsWith('/.well-known')) {
		return resolve(event);
	}

	return paraglideMiddleware(event.request, async ({ request, locale }) => {
		event.request = request;

		const response = await resolve(event, {
			transformPageChunk: ({ html }) => html.replace('%paraglide.lang%', locale)
		});

		if (response.status === 404) {
			throw redirect(307, '/404');
		}

		return response;
	});
};
