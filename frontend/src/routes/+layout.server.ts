import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	// Nur client-seitige Auth-Prüfung
	return {};
};
