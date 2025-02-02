<script lang="ts">
	import { onMount } from "svelte";
	export let selectedValue: string = "";
	export let items: string[];

	let isOpen = false;
	let searchQuery = "";
	let filteredItems = items;
	let highlightedIndex = -1;

	// Function to truncate long headers for display
	function truncateLabel(label: string, maxLength: number = 20): string {
		if (label.length <= maxLength) return label;
		const ellipsis = "...";
		const keepEachSide = Math.floor((maxLength - ellipsis.length) / 2);
		return (
			label.slice(0, keepEachSide) + ellipsis + label.slice(-keepEachSide)
		);
	}

	// Function to filter items based on search query
	function updateFilteredItems() {
		filteredItems = items.filter((item) =>
			item.toLowerCase().includes(searchQuery.toLowerCase()),
		);
		highlightedIndex = -1; // Reset highlight when filtering
	}

	// Function to handle selection
	function selectItem(item: string) {
		selectedValue = item; // Store full value
		searchQuery = truncateLabel(item, 20); // Show truncated version in input
		isOpen = false; // Close dropdown
	}

	// Handle keyboard navigation
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "ArrowDown") {
			event.preventDefault();
			highlightedIndex = Math.min(
				highlightedIndex + 1,
				filteredItems.length - 1,
			);
		} else if (event.key === "ArrowUp") {
			event.preventDefault();
			highlightedIndex = Math.max(highlightedIndex - 1, 0);
		} else if (event.key === "Enter" && highlightedIndex >= 0) {
			selectItem(filteredItems[highlightedIndex]);
		} else if (event.key === "Escape") {
			isOpen = false;
		}
	}

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		if (!(event.target as HTMLElement).closest(".combo-box")) {
			isOpen = false;
		}
	}

	onMount(() => {
		document.addEventListener("click", handleClickOutside);
	});
</script>

<div class="combo-box relative w-full">
	<!-- Input Field -->
	<input
		type="text"
		bind:value={searchQuery}
		placeholder="Select an option..."
		class="w-full px-3 py-2 border rounded-md focus:outline-none"
		on:input={updateFilteredItems}
		on:focus={() => (isOpen = true)}
		on:keydown={handleKeydown}
	/>

	<!-- Dropdown Menu -->
	{#if isOpen && filteredItems.length > 0}
		<ul
			class="dropdown-menu absolute w-full bg-white border rounded-md mt-1 max-h-40 overflow-y-auto shadow-lg z-10"
		>
			{#each filteredItems as item, index}
				<li
					class="text-black px-3 py-2 cursor-pointer hover:bg-gray-200"
					class:selected={index === highlightedIndex}
					on:click={() => selectItem(item)}
				>
					{truncateLabel(item, 20)}
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	@reference 'tailwindcss';

	.combo-box input {
		@apply border border-gray-300 rounded-md py-2 px-3 bg-gray-200 text-black;
	}

	.dropdown-menu li.selected {
		@apply bg-blue-300;
	}
</style>
