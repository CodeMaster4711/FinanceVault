const API_BASE_URL = 'http://localhost:8000/api';

export interface Subscription {
	id: string;
	user_id: string;
	name: string;
	amount: number;
	billing_cycle: string;
	next_billing_date: string;
	category: string;
	is_active: boolean;
}

export interface CreateSubscriptionRequest {
	name: string;
	amount: number;
	billing_cycle: string;
	next_billing_date: string;
	category: string;
}

export interface UpdateSubscriptionRequest {
	name?: string;
	amount?: number;
	billing_cycle?: string;
	next_billing_date?: string;
	category?: string;
	is_active?: boolean;
}

export class SubscriptionService {
	static async getSubscriptions(token: string): Promise<Subscription[]> {
		const response = await fetch(`${API_BASE_URL}/subscriptions`, {
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			throw new Error('Failed to fetch subscriptions');
		}

		return await response.json();
	}

	static async getSubscription(id: string, token: string): Promise<Subscription> {
		const response = await fetch(`${API_BASE_URL}/subscriptions/${id}`, {
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Subscription not found');
			}
			throw new Error('Failed to fetch subscription');
		}

		return await response.json();
	}

	static async createSubscription(
		subscription: CreateSubscriptionRequest,
		token: string
	): Promise<Subscription> {
		const response = await fetch(`${API_BASE_URL}/subscriptions`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${token}`,
			},
			body: JSON.stringify(subscription),
		});

		if (!response.ok) {
			throw new Error('Failed to create subscription');
		}

		return await response.json();
	}

	static async updateSubscription(
		id: string,
		subscription: UpdateSubscriptionRequest,
		token: string
	): Promise<Subscription> {
		const response = await fetch(`${API_BASE_URL}/subscriptions/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${token}`,
			},
			body: JSON.stringify(subscription),
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Subscription not found');
			}
			throw new Error('Failed to update subscription');
		}

		return await response.json();
	}

	static async deleteSubscription(id: string, token: string): Promise<void> {
		const response = await fetch(`${API_BASE_URL}/subscriptions/${id}`, {
			method: 'DELETE',
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Subscription not found');
			}
			throw new Error('Failed to delete subscription');
		}
	}
}
