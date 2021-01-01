<script>
  import Tailwindcss from "./Tailwindcss.svelte";
  import Calculator from "./Calculator.svelte";
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

  <p class="my-2">
    This is the CostOf.Life calculator (
    <a
      href="https://www.investopedia.com/terms/w/workinprogress.asp#:~:text=The%20term%20work%2Din%2Dprogress,stages%20of%20the%20production%20process.">
      WIP
    </a>
    )
  </p>
  <p class="mb-2 text-center">
    Currently the CostOf.Life calculator is available only as a
    <a href="https://github.com/noandrea/costoflife-rs" class="underline">
      command line program
    </a>
    for linux, but it will be made available on this page as soon as possible.
  </p>
  <p class="mb-2 text-center">
    In the meantime you can read about it
    <a href="https://dev.to/noandrea" class="underline">here</a>
    .
  </p>

  <Calculator />

</main>

<!-- footer -->
<footer class="h-10 font-mono text-xs">
  by
  <a class="underline" href="https://adgb.me">adgb</a>

  <span class="ml-2 pl-2 border-l border-costoflife-light-blue">sources</span>
  <a href="https://github.com/noandrea/costoflife-rs" class="underline">
    costoflife-rs
  </a>
  /
  <a href="https://github.com/noandrea/costoflife.ui" class="underline">
    costoflife.ui
  </a>
  <span class="ml-2 pl-2 border-l border-costoflife-light-blue">see also</span>
  <a class="underline" href="https://meetvalis.com">VALIS</a>
</footer>
