import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

type VaultStatus = 'checking' | 'uninitialized' | 'locked' | 'unlocked';

interface VaultState {
	status: VaultStatus;
	error: string | null;
}

const state = writable<VaultState>({ status: 'checking', error: null });

export const vault = {
	subscribe: state.subscribe,

	get status() {
		return get(state).status;
	},

	get error() {
		return get(state).error;
	},

	async init() {
		try {
			const exists = await invoke<boolean>('vault_exists');
			if (!exists) {
				state.set({ status: 'uninitialized', error: null });
				return;
			}
			const locked = await invoke<boolean>('is_locked');
			state.set({ status: locked ? 'locked' : 'unlocked', error: null });
		} catch {
			state.set({ status: 'locked', error: null });
		}
	},

	async setup(passphrase: string): Promise<boolean> {
		state.update((s) => ({ ...s, error: null }));
		try {
			await invoke('setup_vault', { passphrase });
			state.set({ status: 'unlocked', error: null });
			return true;
		} catch (e) {
			state.update((s) => ({ ...s, error: e as string }));
			return false;
		}
	},

	async unlock(passphrase: string, totpCode?: string): Promise<boolean> {
		state.update((s) => ({ ...s, error: null }));
		try {
			await invoke('unlock', { passphrase, totpCode: totpCode ?? null });
			state.set({ status: 'unlocked', error: null });
			return true;
		} catch (e) {
			state.update((s) => ({ ...s, error: e as string }));
			return false;
		}
	},

	async lock() {
		try {
			await invoke('lock');
		} finally {
			state.set({ status: 'locked', error: null });
		}
	},
};

export const isUnlocked = derived(state, (s) => s.status === 'unlocked');
