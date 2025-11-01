const API_BASE_URL = 'http://localhost:8000/api';

export interface Expense {
	id: string;
	user_id: string;
	description: string;
	amount: number;
	date: string;
	category: string;
}

export interface CreateExpenseRequest {
	description: string;
	amount: number;
	date: string;
	category: string;
}

export interface UpdateExpenseRequest {
	description?: string;
	amount?: number;
	date?: string;
	category?: string;
}

export class ExpenseService {
	static async getExpenses(token: string): Promise<Expense[]> {
		const response = await fetch(`${API_BASE_URL}/expenses`, {
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			throw new Error('Failed to fetch expenses');
		}

		return await response.json();
	}

	static async getExpense(id: string, token: string): Promise<Expense> {
		const response = await fetch(`${API_BASE_URL}/expenses/${id}`, {
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Expense not found');
			}
			throw new Error('Failed to fetch expense');
		}

		return await response.json();
	}

	static async createExpense(
		expense: CreateExpenseRequest,
		token: string
	): Promise<Expense> {
		const response = await fetch(`${API_BASE_URL}/expenses`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${token}`,
			},
			body: JSON.stringify(expense),
		});

		if (!response.ok) {
			throw new Error('Failed to create expense');
		}

		return await response.json();
	}

	static async updateExpense(
		id: string,
		expense: UpdateExpenseRequest,
		token: string
	): Promise<Expense> {
		const response = await fetch(`${API_BASE_URL}/expenses/${id}`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${token}`,
			},
			body: JSON.stringify(expense),
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Expense not found');
			}
			throw new Error('Failed to update expense');
		}

		return await response.json();
	}

	static async deleteExpense(id: string, token: string): Promise<void> {
		const response = await fetch(`${API_BASE_URL}/expenses/${id}`, {
			method: 'DELETE',
			headers: {
				'Authorization': `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			if (response.status === 404) {
				throw new Error('Expense not found');
			}
			throw new Error('Failed to delete expense');
		}
	}
}
