<script lang="ts">
  interface Props {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
  }

  let {
    value = $bindable(0.005),
    min = 0.0001,
    max = 0.05,
    step = 0.0001,
  }: Props = $props();

  function handleInput(e: Event) {
    const num: number = parseFloat((e.target as HTMLInputElement).value);
    if (!isNaN(num) && num >= min && num <= max) {
      value = num;
    }
  }
</script>

<div class="contrast-slide">
  <label class="text-md text-black">Contrast</label>
  <input
    class="cursor-pointer"
    type="range"
    {min}
    {max}
    {step}
    {value}
    oninput={handleInput}
  />
  <input
    class="text-xs bg-white text-black rounded-sm px-1"
    type="number"
    {min}
    {max}
    {step}
    {value}
    oninput={handleInput}
  />
  <span class="text-xs text-black">+/- {value}</span>
</div>

<style>
  @reference 'tailwindcss';
  .contrast-slide {
    @apply flex items-center space-x-1 bg-blue-400 px-2 rounded-sm border-1;
  }
</style>
