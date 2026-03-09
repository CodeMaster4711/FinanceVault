import { invoke } from '@tauri-apps/api/core';

export interface Expense {
	id: string;
	title: string;
	amount: number;
	currency: string;
	category: string;
	date: string;
	created_at: string;
}

export interface CreateExpense {
	title: string;
	amount: number;
	currency?: string;
	category: string;
	date: string;
}

export interface UpdateExpense {
	title?: string;
	amount?: number;
	currency?: string;
	category?: string;
	date?: string;
}

export const ExpenseService = {
	getAll: () => invoke<Expense[]>('get_expenses'),
	create: (input: CreateExpense) => invoke<Expense>('create_expense', { input }),
	update: (id: string, input: UpdateExpense) => invoke<void>('update_expense', { id, input }),
	delete: (id: string) => invoke<void>('delete_expense', { id }),
};
