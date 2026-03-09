import { invoke } from '@tauri-apps/api/core';

export interface Quote {
	ticker: string;
	name: string;
	price: number;
	currency: string;
	change_pct: number;
}

export const YahooService = {
	fetchQuotes: (tickers: string[]) =>
		invoke<Quote[]>('fetch_quotes', { tickers }),
};
