import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { ApiClient } from '$lib/services/api-client';

interface User {
	id: string;
	username: string;
}

interface AuthState {
	user: User | null;
	token: string | null;
	isAuthenticated: boolean;
	isLoading: boolean;
}

const initialState: AuthState = {
	user: null,
	token: null,
	isAuthenticated: false,
	isLoading: false
};

// Helper function to get cookie value
function getCookie(name: string): string | null {
	if (!browser) return null;
	
	const value = `; ${document.cookie}`;
	const parts = value.split(`; ${name}=`);
	if (parts.length === 2) {
		return parts.pop()?.split(';').shift() || null;
	}
	return null;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	return {
		subscribe,
		init: (user: User | null) => {
			if (browser) {
				const token = getCookie('auth_token');
				if (user && token) {
					set({
						user,
						token,
						isAuthenticated: true,
						isLoading: false
					});
					// Start periodic session validation
					ApiClient.startSessionCheck();
				} else {
					set(initialState);
					ApiClient.stopSessionCheck();
				}
			}
		},
		login: (user: User, token: string) => {
			set({
				user,
				token,
				isAuthenticated: true,
				isLoading: false
			});
			// Start periodic session validation
			if (browser) {
				ApiClient.startSessionCheck();
			}
		},
		logout: () => {
			// Stop periodic session validation
			if (browser) {
				ApiClient.stopSessionCheck();
			}
			// Cookie is deleted by the /api/set-auth-cookie endpoint
			set(initialState);
		},
		setLoading: (loading: boolean) => {
			update(state => ({ ...state, isLoading: loading }));
		}
	};
}

export const authStore = createAuthStore();