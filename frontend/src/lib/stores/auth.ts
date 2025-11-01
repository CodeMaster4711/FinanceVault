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
		login: (user: User, token: string) => {
			if (browser) {
				localStorage.setItem('user', JSON.stringify(user));
				// Token wird vom Backend in Cookies gesetzt
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
				localStorage.removeItem('user');
				// Cookie wird vom Backend gelöscht
			}
			set(initialState);
		},
		setLoading: (loading: boolean) => {
			update(state => ({ ...state, isLoading: loading }));
		},
		initialize: () => {
			if (browser) {
				// Token aus Cookies lesen
				const token = getCookie('auth_token');
				const userStr = localStorage.getItem('user');
				
				console.log('Auth store initializing...', { token: token ? 'present' : 'missing', user: userStr ? 'present' : 'missing' });
				
				if (token && userStr) {
					try {
						const user = JSON.parse(userStr);
						console.log('Auth store initialized with user:', user);
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
				} else {
					console.log('Auth store: No token or user found, setting initial state');
					set(initialState);
				}
			}
		}
	};
}

export const authStore = createAuthStore();