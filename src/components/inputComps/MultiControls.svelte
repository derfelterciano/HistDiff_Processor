<script lang="ts">
	import ControlSelector from "./ControlSelector.svelte";
	import type { ControlDefinition } from "../../types/controlType";

	export let controls: (ControlDefinition | null)[] = [];

	const addControl = () => {
		controls = [...controls, null];
	};

	const updateControl = (id: number, item: ControlDefinition) => {
		controls[id] = item;
		controls = controls;
	};

	const deleteControl = (id: number) => {
		controls = controls.filter((_, i) => i !== id);
	};
</script>

<div class="multi-control">
	<h6>Additional Controls</h6>
</div>
