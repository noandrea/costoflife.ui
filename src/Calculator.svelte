<script>
  import { costoflife_per_diem } from "costoflife";

  // what is going on
  let raw = "";
  let perDiem = null;

  function handleKeydown(event) {
    // console.log("Hello ", costoflife_greetings());
    if (event.keyCode == 13) {
      console.log("key pressed: ", raw);
      // this is enter
      // perDiem = costoflife_per_diem(raw);
      const redirectURL = "https://dev.to/noandrea";
      const x = raw;
      // const pm = module.costoflife_per_diem(x);
      perDiem = `Deeply sorry, it is not yet ready, <a href="${redirectURL}" class="underline">here's why</a>`;
      console.log("key pressed: ", pm);
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="text-center">
  <input
    bind:value={raw}
    type="text"
    id="record"
    name="record"
    placeholder="Rent 923€ 1m12x #rent"
    class="text-center w-max bg-gray-100 rounded border border-gray-300
    focus:border-costoflife-light-blue text-base outline-none text-gray-700 py-1
    px-3 leading-8 transition-colors duration-200 ease-in-out" />

  <div>
    {#if perDiem}
      <p class="text-lg">
        {@html perDiem}
      </p>
    {:else}
      <p>O_o</p>
    {/if}
  </div>
</div>
