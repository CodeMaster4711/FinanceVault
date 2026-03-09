import { invoke } from '@tauri-apps/api/core';

export interface Quote {
	isin: string;
	ticker: string;
	name: string;
	price: number;
	currency: string;
	change_pct: number;
}

export interface FundHolding {
	symbol: string;
	name: string;
	percent: number;
}

export interface SectorWeight {
	sector: string;
	percent: number;
}

export interface FundData {
	ticker: string;
	holdings: FundHolding[];
	sector_weights: SectorWeight[];
}

export interface PricePoint {
	timestamp: number;
	close: number;
}

export const YahooService = {
	resolveIsin: (isin: string) =>
		invoke<[string, string]>('resolve_isin', { isin }),

	fetchQuotes: (positions: [string, string][]) =>
		invoke<Quote[]>('fetch_quotes', { positions }),

	fetchFundData: (ticker: string) =>
		invoke<FundData>('fetch_fund_data', { ticker }),

	fetchHistory: (ticker: string, range: string) =>
		invoke<PricePoint[]>('fetch_history', { ticker, range }),
};
