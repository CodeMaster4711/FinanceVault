import { invoke } from '@tauri-apps/api/core';

export interface Quote {
	isin: string;
	ticker: string;
	name: string;
	price: number;
	currency: string;
	change_pct: number;
}

export const YahooService = {
	resolveIsin: (isin: string) =>
		invoke<[string, string]>('resolve_isin', { isin }),

	fetchQuotes: (positions: [string, string][]) =>
		invoke<Quote[]>('fetch_quotes', { positions }),
};
