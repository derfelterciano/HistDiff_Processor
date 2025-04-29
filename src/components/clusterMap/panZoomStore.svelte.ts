import { writable } from "svelte/store";

export interface PanZoom {
	scale: number;

	tx: number;
	ty: number;

	minScale: number;
	maxScale: number;
}

function createPanZoomStore() {
	const inital: PanZoom = {
		scale: 1,
		tx: 0,
		ty: 0,
		minScale: 0.2,
		maxScale: 10,
	};
	const { subscribe, update, set } = writable<PanZoom>(inital);

	return {
		subscribe,
		zoom: (factor: number) =>
			update((s) => {
				const newScale = Math.min(
					s.maxScale,
					Math.max(s.minScale, s.scale * factor),
				);
				return { ...s, scale: newScale };
			}),
		pan: (dx: number, dy: number) =>
			update((s) => ({
				...s,
				tx: s.tx + dx,
				ty: s.ty + dy,
			})),

		reset: () => set(inital),

		setMinScale: (min: number) =>
			update((s) => {
				const clampedScale = Math.max(
					min,
					Math.min(s.scale, s.maxScale),
				);
				return { ...s, minScale: min, scale: clampedScale };
			}),

		set,
	};
}

export const panZoom = createPanZoomStore();
