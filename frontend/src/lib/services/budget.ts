import { invoke } from '@tauri-apps/api/core';

export interface BudgetMonth {
	id: string;
	month: string;
	income: f64;
	created_at: string;
}

export interface BudgetSummary {
	month: string;
	income: number;
	total_expenses: number;
	total_subscriptions_monthly: number;
	remaining: number;
}

type f64 = number;

export const BudgetService = {
	getMonths: () => invoke<BudgetMonth[]>('get_budget_months'),
	upsert: (month: string, income: number) =>
		invoke<BudgetMonth>('upsert_budget_month', { month, income }),
	getSummary: (month: string) =>
		invoke<BudgetSummary>('get_budget_summary', { month }),
};
