<script lang="ts">
  interface Props {
    options?: string[];
    value?: string;
    inputValue?: string;
    showList?: boolean;
  }

  let {
    options = $bindable([]),
    value = $bindable(""),
    inputValue = $bindable(value),
    showList = false,
  }: Props = $props();

  const filtered = $derived(() => {
    return options.filter((option) =>
      option.toLowerCase().includes(inputValue.toLowerCase())
    );
  });

  $effect(() => {
    $inspect(options);
    $inspect(filtered());
    $inspect(inputValue);
  });

  function choose(option: string) {
    value = option;
    inputValue = option;
    showList = false;
  }

  function onInput(e: Event) {
    inputValue = (e.target as HTMLInputElement).value;
    showList = true;
    value = "";
  }

  function onBlur() {
    setTimeout(() => (showList = false), 100);
  }
</script>

<div class="combobox-div relative w-full">
  <input
    type="text"
    class="flex border-2 border-indigo-500 rounded-md bg-white text-black text-center w-full"
    bind:value={inputValue}
    oninput={onInput}
    onfocus={() => (showList = true)}
    onblur={onBlur}
    autocomplete="off"
    placeholder="Select or type ..."
  />

  {#if showList && filtered().length > 0}
    <ul
      class="absolute z-10 bg-white w-full mt-1 rounded shadow max-h-40 overflow-auto"
    >
      {#each filtered() as option}
        <li>
          <button
            type="button"
            class="w-full text-left px-2 py-1 hover:bg-blue-100 cursor-pointer text-black"
            onmousedown={() => choose(option)}
          >
            {option}
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>
