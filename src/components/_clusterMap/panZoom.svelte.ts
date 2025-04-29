export interface PanZoomState {
	scale: number;
	tx: number;
	ty: number;
	minScale: number;
	maxScale: number;
}

/**
 * Connect wheel and panning functions
 * Calls redraw() when view changes
 */
export function initPanZoom(
	canvas: HTMLCanvasElement,
	state: PanZoomState,
	redraw: () => void,
): () => void {
	/**Scroll wheel functionality */
	function onWheel(e: WheelEvent) {
		e.preventDefault();
		const rec = canvas.getBoundingClientRect();
		const mx = e.clientX - rec.left;
		const my = e.clientY - rec.top;
		const factor = e.deltaY < 0 ? 1.1 : 0.9;
		const newScale = Math.min(
			state.maxScale,
			Math.max(state.minScale, state.scale * factor),
		);
		const actual = newScale / state.scale;
		state.scale = newScale;
		state.tx = mx - actual * (mx - state.tx);
		state.ty = my - actual * (my - state.ty);
		redraw();
	}

	let dragging = false,
		sx = 0,
		sy = 0;
	function onMouseDown(e: MouseEvent) {
		dragging = true;
		sx = e.clientX;
		sy = e.clientY;
	}

	function onMouseMove(e: MouseEvent) {
		if (!dragging) return;

		const dx = e.clientX - sx;
		const dy = e.clientY - sy;
		state.tx += dx;
		state.ty += dy;
		sx = e.clientX;
		sy = e.clientY;
		redraw();
	}

	function onMouseUp() {
		dragging = false;
	}

	canvas.addEventListener("wheel", onWheel, { passive: false });
	canvas.addEventListener("mousedown", onMouseDown);

	window.addEventListener("mousemove", onMouseMove);
	window.addEventListener("mouseup", onMouseUp);

	return () => {
		canvas.removeEventListener("wheel", onWheel);
		canvas.removeEventListener("mousedown", onMouseDown);
		window.removeEventListener("mousemove", onMouseMove);
		window.removeEventListener("mouseup", onMouseUp);
	};
}
