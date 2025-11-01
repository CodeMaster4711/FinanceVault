import { writable } from 'svelte/store';
import { browser } from '$app/environment';

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
				} else {
					set(initialState);
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
		},
		logout: () => {
			// Cookie is deleted by the /api/set-auth-cookie endpoint
			set(initialState);
		},
		setLoading: (loading: boolean) => {
			update(state => ({ ...state, isLoading: loading }));
		}
	};
}

export const authStore = createAuthStore();