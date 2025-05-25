import { json } from '@sveltejs/kit';

export const GET = async ({ cookies }) => {
	const user = cookies.get('user');
	return json({ loggedIn: !!user });
};