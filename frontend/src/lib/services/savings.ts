import { invoke } from '@tauri-apps/api/core';

export interface SavingsPlan {
	id: string;
	name: string;
	isin: string;
	ticker: string;
	amount: number;
	currency: string;
	interval: string;
	next_date: string;
	created_at: string;
}

export interface CreateSavingsPlan {
	name: string;
	isin: string;
	ticker: string;
	amount: number;
	currency?: string;
	interval: string;
	next_date: string;
}

export interface UpdateSavingsPlan {
	name?: string;
	amount?: number;
	currency?: string;
	interval?: string;
	next_date?: string;
}

export const SavingsService = {
	getAll: () => invoke<SavingsPlan[]>('get_savings_plans'),
	create: (input: CreateSavingsPlan) => invoke<SavingsPlan>('create_savings_plan', { input }),
	update: (id: string, input: UpdateSavingsPlan) => invoke<void>('update_savings_plan', { id, input }),
	delete: (id: string) => invoke<void>('delete_savings_plan', { id }),
};
