import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const { token } = await request.json();
	
	if (!token) {
		return json({ error: 'Token required' }, { status: 400 });
	}
	
	// Set HTTP-only cookie
	cookies.set('auth_token', token, {
		path: '/',
		httpOnly: true,
		sameSite: 'strict',
		secure: false, // Set to true in production with HTTPS
		maxAge: 60 * 60 * 24 * 7 // 7 days
	});
	
	return json({ success: true });
};
