import { writable } from "svelte/store";

export interface PanZoom {
	scale: number;

	tx: number;
	ty: number;

	minScale: number;
	maxScale: number;

	// bounding clamps
	contentW: number;
	contentH: number;
	viewW: number;
	viewH: number;
}

function createPanZoomStore() {
	const inital: PanZoom = {
		scale: 1,
		tx: 0,
		ty: 0,
		minScale: 0.2,
		maxScale: 100,

		contentW: 0,
		contentH: 0,
		viewW: 0,
		viewH: 0,
	};
	const { subscribe, update, set } = writable<PanZoom>(inital);

	return {
		subscribe,
		setBounds: (
			contentW: number,
			contentH: number,
			viewW: number,
			viewH: number,
		) => update((s) => {
			return { ...s, contentW, contentH, viewW, viewH };
		}),
		zoom: (factor: number) =>
			update((s) => {
				// clamp scaling
				const newScale = Math.min(
					s.maxScale,
					Math.max(s.minScale, s.scale * factor),
				);

				const minTx = Math.min(0, s.viewW - s.contentW * newScale);
				const maxTx = 0;
				const tx = Math.min(maxTx, Math.max(minTx, s.tx));

				const minTy = Math.min(0, s.viewH - s.contentH * newScale);
				const maxTy = 0;
				const ty = Math.min(maxTy, Math.max(minTy, s.ty));

				return { ...s, scale: newScale, ty: ty, tx: tx };
			}),
		pan: (dx: number, dy: number) =>
			update((s) => {
				const minTx = Math.min(0, s.viewW - s.contentW * s.scale);
				const maxTx = 0;
				const tx = Math.min(maxTx, Math.max(minTx, s.tx + dx));

				const minTy = Math.min(0, s.viewH - s.contentH * s.scale);
				const maxTy = 0;
				const ty = Math.min(maxTy, Math.max(minTy, s.ty + dy));

				return { ...s, tx: tx, ty: ty };
			}),

		reset: () =>
			update((s) => {
				return {
					...s,
					scale: 1,
					tx: 0,
					ty: 0,
				};
			}),

		setMinScale: (min: number) =>
			update((s) => {
				const clampedScale = Math.max(
					min,
					Math.min(s.scale, s.maxScale),
				);
				return { ...s, minScale: min, scale: clampedScale };
			}),

		set,

		zoomAt: (factor: number, cx: number, cy: number) =>
			update((s) => {
				const newScale = Math.min(
					s.maxScale,
					Math.max(s.minScale, s.scale * factor),
				);
				const actual = newScale / s.scale;

				// move towards cx, cy
				let tx = cx - actual * (cx - s.tx);
				let ty = cy - actual * (cy - s.ty);

				// clamp to bounds
				const minTx = Math.min(0, s.viewW - s.contentW * newScale);
				tx = Math.min(0, Math.max(minTx, tx));

				const minTy = Math.min(0, s.viewH - s.contentH * newScale);
				ty = Math.min(0, Math.max(minTy, ty));

				return { ...s, scale: newScale, tx, ty };
			}),
	};
}

export const panZoom = createPanZoomStore();
