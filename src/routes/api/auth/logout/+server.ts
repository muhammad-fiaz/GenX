import { json } from '@sveltejs/kit';

export const POST = async ({ cookies }) => {
	// Delete the 'user' cookie by setting it to empty and maxAge to 0
	cookies.set('user', '', {
		path: '/',
		maxAge: 0,
		httpOnly: true,
		sameSite: 'strict',
		secure: process.env.NODE_ENV === 'production',
	});

	return json({ success: true });
};