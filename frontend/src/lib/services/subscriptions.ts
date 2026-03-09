import { invoke } from '@tauri-apps/api/core';

export interface Subscription {
	id: string;
	name: string;
	amount: number;
	currency: string;
	billing: string;
	next_billing: string;
	created_at: string;
}

export interface CreateSubscription {
	name: string;
	amount: number;
	currency?: string;
	billing: string;
	next_billing: string;
}

export interface UpdateSubscription {
	name?: string;
	amount?: number;
	currency?: string;
	billing?: string;
	next_billing?: string;
}

export const SubscriptionService = {
	getAll: () => invoke<Subscription[]>('get_subscriptions'),
	create: (input: CreateSubscription) => invoke<Subscription>('create_subscription', { input }),
	update: (id: string, input: UpdateSubscription) => invoke<void>('update_subscription', { id, input }),
	delete: (id: string) => invoke<void>('delete_subscription', { id }),
};
