import { redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent }) => {
    const { user } = await parent();
    
    // If user is already authenticated, redirect to expenses
    if (user) {
        throw redirect(303, '/expenses/monthly');
    }
    
    // If not authenticated, redirect to signin
    throw redirect(303, '/signin');
};

export const actions = {
	logout: async ({ cookies }) => {
		// Remove auth cookie
		cookies.delete('auth_token', { path: '/' });
		
		// Redirect to signin
		throw redirect(303, '/signin');
	}
} satisfies Actions;
