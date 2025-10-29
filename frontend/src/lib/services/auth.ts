// @ts-ignore
import * as forge from 'node-forge';

const API_BASE_URL = 'http://localhost:8000/api';

interface RegisterRequest {
	username: string;
	encrypted_password: string;
}

interface LoginRequest {
	username: string;
	encrypted_password: string;
}

interface AuthResponse {
	token: string;
	user_id: string;
	username: string;
}

interface PublicKeyResponse {
	public_key: string;
}

export class AuthService {
	private static publicKey: string | null = null;

	static async getPublicKey(): Promise<string> {
		if (this.publicKey) {
			return this.publicKey;
		}

		const response = await fetch(`${API_BASE_URL}/public-key`);
		if (!response.ok) {
			throw new Error('Failed to get public key');
		}

		const data: PublicKeyResponse = await response.json();
		this.publicKey = data.public_key;
		return this.publicKey;
	}

	static encryptPassword(password: string, publicKey: string): string {
		const publicKeyObj = forge.pki.publicKeyFromPem(publicKey);
		const encrypted = publicKeyObj.encrypt(password, 'RSA-OAEP');
		return forge.util.encode64(encrypted);
	}

	static async register(username: string, password: string): Promise<AuthResponse> {
		const publicKey = await this.getPublicKey();
		const encryptedPassword = this.encryptPassword(password, publicKey);

		const request: RegisterRequest = {
			username,
			encrypted_password: encryptedPassword
		};

		const response = await fetch(`${API_BASE_URL}/register`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(request),
		});

		if (!response.ok) {
			if (response.status === 409) {
				throw new Error('User already exists');
			}
			throw new Error('Registration failed');
		}

		return await response.json();
	}

	static async login(username: string, password: string): Promise<AuthResponse> {
		const publicKey = await this.getPublicKey();
		const encryptedPassword = this.encryptPassword(password, publicKey);

		const request: LoginRequest = {
			username,
			encrypted_password: encryptedPassword
		};

		const response = await fetch(`${API_BASE_URL}/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(request),
		});

		if (!response.ok) {
			if (response.status === 401) {
				throw new Error('Invalid credentials');
			}
			throw new Error('Login failed');
		}

		return await response.json();
	}

	static async logout(token: string): Promise<void> {
		const response = await fetch(`${API_BASE_URL}/logout`, {
			method: 'POST',
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok && response.status !== 401) {
			throw new Error('Logout failed');
		}
	}
}