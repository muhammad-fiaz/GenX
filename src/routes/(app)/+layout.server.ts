import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const user = cookies.get('user');

	if (!user) {
		throw redirect(303, '/signin');
	}

	return { user };
};
