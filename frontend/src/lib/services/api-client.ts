import { PUBLIC_API_BASE_URL } from '$env/static/public';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth';

const API_BASE_URL = PUBLIC_API_BASE_URL;

export interface ApiError extends Error {
	status: number;
	statusText: string;
}

export class ApiClient {
	private static sessionCheckInterval: number | null = null;
	private static isSessionCheckRunning = false;

	/**
	 * Makes an authenticated API request with automatic error handling
	 * and logout on 401 errors
	 */
	static async fetch(
		url: string,
		options: RequestInit = {},
		token?: string
	): Promise<Response> {
		// Get token from cookie if not provided
		if (!token && browser) {
			const cookieToken = this.getTokenFromCookie();
			if (cookieToken) {
				token = cookieToken;
			}
		}

		// Add authorization header if token exists
		const headers: Record<string, string> = {};
		
		if (options.headers) {
			if (options.headers instanceof Headers) {
				options.headers.forEach((value, key) => {
					headers[key] = value;
				});
			} else if (Array.isArray(options.headers)) {
				options.headers.forEach(([key, value]) => {
					headers[key] = value;
				});
			} else {
				Object.assign(headers, options.headers);
			}
		}

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		try {
			const response = await fetch(`${API_BASE_URL}${url}`, {
				...options,
				headers,
				credentials: 'include', // Important for cookies
			});

			// Handle 401 Unauthorized - session expired or invalid
			if (response.status === 401) {
				console.log('Received 401, logging out user');
				await this.handleUnauthorized();
				
				const error = new Error('Unauthorized') as ApiError;
				error.status = 401;
				error.statusText = 'Unauthorized';
				throw error;
			}

			return response;
		} catch (error) {
			// Re-throw API errors
			if ((error as ApiError).status === 401) {
				throw error;
			}

			// Network or other errors
			console.error('API request failed:', error);
			throw error;
		}
	}

	/**
	 * Helper for GET requests
	 */
	static async get(url: string, token?: string): Promise<Response> {
		return this.fetch(url, { method: 'GET' }, token);
	}

	/**
	 * Helper for POST requests
	 */
	static async post(url: string, body?: unknown, token?: string): Promise<Response> {
		return this.fetch(
			url,
			{
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: body ? JSON.stringify(body) : undefined,
			},
			token
		);
	}

	/**
	 * Helper for PUT requests
	 */
	static async put(url: string, body?: unknown, token?: string): Promise<Response> {
		return this.fetch(
			url,
			{
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: body ? JSON.stringify(body) : undefined,
			},
			token
		);
	}

	/**
	 * Helper for DELETE requests
	 */
	static async delete(url: string, token?: string): Promise<Response> {
		return this.fetch(url, { method: 'DELETE' }, token);
	}

	/**
	 * Validates the current session with the backend
	 */
	static async validateSession(): Promise<boolean> {
		if (!browser) return false;

		const token = this.getTokenFromCookie();
		if (!token) {
			return false;
		}

		try {
			const response = await this.get('/validate-session', token);
			return response.ok;
		} catch (error) {
			console.error('Session validation failed:', error);
			return false;
		}
	}

	/**
	 * Starts periodic session validation (every 5 minutes)
	 */
	static startSessionCheck() {
		if (!browser || this.sessionCheckInterval !== null) {
			return;
		}

		console.log('Starting periodic session validation');

		// Check immediately
		this.performSessionCheck();

		// Then check every 5 minutes
		this.sessionCheckInterval = window.setInterval(() => {
			this.performSessionCheck();
		}, 5 * 60 * 1000); // 5 minutes
	}

	/**
	 * Stops periodic session validation
	 */
	static stopSessionCheck() {
		if (this.sessionCheckInterval !== null) {
			console.log('Stopping periodic session validation');
			clearInterval(this.sessionCheckInterval);
			this.sessionCheckInterval = null;
		}
	}

	/**
	 * Performs a session check
	 */
	private static async performSessionCheck() {
		if (this.isSessionCheckRunning) {
			return;
		}

		this.isSessionCheckRunning = true;

		try {
			const isValid = await this.validateSession();
			
			if (!isValid) {
				console.log('Session validation failed, logging out');
				await this.handleUnauthorized();
			} else {
				console.log('Session is still valid');
			}
		} catch (error) {
			console.error('Error during session check:', error);
		} finally {
			this.isSessionCheckRunning = false;
		}
	}

	/**
	 * Handles unauthorized access - logs out user and redirects to signin
	 */
	private static async handleUnauthorized() {
		console.log('Handling unauthorized access');
		
		// Stop session checks
		this.stopSessionCheck();

		// Clear auth cookie
		await fetch('/api/set-auth-cookie', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ token: null }),
		});

		// Update auth store
		authStore.logout();

		// Redirect to signin page
		if (browser && window.location.pathname !== '/signin') {
			goto('/signin');
		}
	}

	/**
	 * Gets token from cookie
	 */
	private static getTokenFromCookie(): string | null {
		if (!browser) return null;

		const value = `; ${document.cookie}`;
		const parts = value.split(`; auth_token=`);
		if (parts.length === 2) {
			return parts.pop()?.split(';').shift() || null;
		}
		return null;
	}
}
