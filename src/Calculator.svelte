<script>
  import { costoflife_per_diem } from "costoflife";
  // what is going on
  let raw = "";
  let perDiem = -1;

  function handleKeydown(event) {
    // console.log("Hello ", costoflife_greetings());
    if (event.keyCode == 13) {
      console.log("key pressed: ", raw);
      // this is enter
      // perDiem = costoflife_per_diem(raw);
      const x = raw;
      const pm = costoflife_per_diem(x);
      console.log("key pressed: ", pm);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="">
  <input
    bind:value={raw}
    type="text"
    id="record"
    name="record"
    placeholder="Rent 923€ 1m12x #rent"
    class="text-center w-max bg-gray-100 rounded border border-gray-300
    focus:border-costoflife-light-blue text-base outline-none text-gray-700 py-1
    px-3 leading-8 transition-colors duration-200 ease-in-out" />

  <div class="text-center">
    {#if perDiem >= 0}
      <p class="text-lg">{perDiem}€</p>
    {:else}
      <p>O_o</p>
    {/if}
  </div>
</div>
