import { json } from '@sveltejs/kit';

export const POST = async ({ request, cookies }) => {
	const user = await request.json();

	const cookieUser = {
		id: user.node_id,
		githubId: user.id,
		username: user.login,
		avatarUrl: user.avatar_url,
		email: user.email,
		name: user.name
	};

	// Set cookie for 7 days, httpOnly, secure in production, sameSite strict
	cookies.set('user', JSON.stringify(cookieUser), {
		path: '/',
		httpOnly: true, // only accessible by server
		maxAge: 60 * 60 * 24 * 7, // 7 days
		sameSite: 'strict',
		secure: process.env.NODE_ENV === 'production'
	});

	return json({ success: true });
};