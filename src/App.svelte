<script>
  import Tailwindcss from "./Tailwindcss.svelte";
  export let name;
</script>

<Tailwindcss />

<main class="flex flex-grow flex-col justify-center items-center">
  <div>
    <img
      width="518"
      height="518"
      src="/assets/home.svg"
      alt="CostOf.Life Logo" />

    <img
      class="mt-4"
      width="518"
      height="90"
      src="/assets/logo-text.svg"
      alt="CostOf.Life Logo" />

  </div>

  <a
    class="font-mono text-sm mt-2"
    href="https://github.com/noandrea/costoflife-rs">
    github.com/noandrea/costoflife-rs
  </a>

</main>

<!-- footer -->
<footer class="h-10 font-mono text-xs">
  by
  <a class="underline" href="https://adgb.me">adgb</a>
  <span class="ml-2 pl-2 border-l border-costoflife-light-blue">see also</span>
  <a class="underline" href="https://meetvalis.com">VALIS</a>
</footer>
