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

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	return {
		subscribe,
		init: (user: User | null, token: string | null) => {
			console.log('[auth.ts] Initializing authStore...', { user, token });
			if (browser) {
				if (user && token) {
					console.log('[auth.ts] User and token found, setting auth state.');
					set({
						user,
						token,
						isAuthenticated: true,
						isLoading: false
					});
					// Start periodic session validation
					ApiClient.startSessionCheck();
				} else {
					console.log('[auth.ts] No user or token, setting initial state.');
					set(initialState);
					ApiClient.stopSessionCheck();
				}
			}
		},
		login: (user: User, token: string) => {
			console.log('[auth.ts] Logging in user:', user);
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
			console.log('[auth.ts] Logging out user.');
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