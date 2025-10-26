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

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	return {
		subscribe,
		login: (user: User, token: string) => {
			if (browser) {
				localStorage.setItem('auth_token', token);
				localStorage.setItem('user', JSON.stringify(user));
			}
			set({
				user,
				token,
				isAuthenticated: true,
				isLoading: false
			});
		},
		logout: () => {
			if (browser) {
				localStorage.removeItem('auth_token');
				localStorage.removeItem('user');
			}
			set(initialState);
		},
		setLoading: (loading: boolean) => {
			update(state => ({ ...state, isLoading: loading }));
		},
		initialize: () => {
			if (browser) {
				const token = localStorage.getItem('auth_token');
				const userStr = localStorage.getItem('user');
				
				if (token && userStr) {
					try {
						const user = JSON.parse(userStr);
						set({
							user,
							token,
							isAuthenticated: true,
							isLoading: false
						});
					} catch (error) {
						console.error('Failed to parse stored user data:', error);
						set(initialState);
					}
				}
			}
		}
	};
}

export const authStore = createAuthStore();