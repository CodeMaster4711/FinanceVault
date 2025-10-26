import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies, url }) => {
	const token = cookies.get('auth_token');
	const publicPaths = ['/signin', '/signup'];
	
	// If no token and not on a public path, redirect to signin
	if (!token && !publicPaths.includes(url.pathname)) {
		throw redirect(302, '/signin');
	}
	
	// If token exists and user is on signin/signup, redirect to home
	if (token && publicPaths.includes(url.pathname)) {
		throw redirect(302, '/');
	}
	
	return {
		user: token ? { authenticated: true } : null
	};
};
