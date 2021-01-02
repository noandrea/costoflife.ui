<script>
  import Tailwindcss from "./Tailwindcss.svelte";
  import Calculator from "./Calculator.svelte";
</script>

<Tailwindcss />

<main
  class="flex flex-grow flex-col lg:flex-row justify-center items-center px-4
  py-12 bg-col-red text-center lg:text-left">

  <!--left column-->
  <div
    class=" border-col-pink pb-8 mb-8 border-b lg:border-b-0 lg:pb-0 lg:mb-0
    lg:w-1/3 lg:pr-6 lg:border-r">

    <h2 class="text-7xl mb-4 tracking-wider font-serif">
      COST OF
      <br />
      LIFE
    </h2>

    <p class="mb-2 mt-4">
      <strong>What is this?</strong>
      CostOf.Life is an experiment in personal finance management loosely based
      on the
      <a
        href="https://www.econlib.org/buying-boots"
        class="underline"
        target="_blank">
        boots theory
      </a>
      .
    </p>

    <p class="mb-2">
      The idea is simple, whenever you have an expense that is constant or
      recurring or when you make a purchase for something that has a substantial
      lifespan, you can split the cost of it over its lifetime and calculate a
      <span class="italic font-semibold">per diem</span>
      .
    </p>

    <p class="mb-2">
      The sum of all
      <span class="italic">per diem</span>
      gives your CostOf.Life baseline. That gives you an idea about how much you
      need to earn to make your life costs sustainable and can help you decide
      whenever an expense is worth making.
    </p>

  </div>

  <div class="flex-grow text-center pl-4">

    <p class="mt-2 mb-4 lg:w-2/3 mx-4 lg:mx-auto">
      This is the CostOf.Life calculator (
      <a
        href="https://www.investopedia.com/terms/w/workinprogress.asp#:~:text=The%20term%20work%2Din%2Dprogress,stages%20of%20the%20production%20process.">
        WIP
      </a>
      )
    </p>

    <div class="lg:max-w-xl mx-4 lg:mx-auto">
      <Calculator />
    </div>

  </div>

</main>

<!-- footer -->

<footer
  class="text-col-white body-font border-t bg-col-blue w-full flex flex-col
  mx-12">
  <div
    class="container py-6 flex md:flex-row md:flex-nowrap flex-wrap flex-col">

    <div
      class="flex-grow flex flex-wrap -mb-10 md:mt-0 mt-10 md:text-left
      text-center">
      <div class="lg:w-1/4 md:w-1/2 w-full px-4">
        <h2
          class="title-font font-medium text-col-pink tracking-widest text-sm
          mb-3 ">
          Links
        </h2>
        <nav class="list-none mb-10">
          <li>
            <a
              class=" hover:text-white-800"
              href="https://dev.to/noandrea/rust-wasm-tailwind-svelte-50hj-temp-slug-8609489?preview=bf84dc534120135e9041328dc9f60347e0aee64e65d737d1adfeedadcb22a8612a3f9090b3efad89e9177f8cee214c30a0d36b82ed43713b73ba6d10">
              Tech article
            </a>
          </li>
        </nav>
      </div>
      <div class="lg:w-1/4 md:w-1/2 w-full px-4">
        <h2
          class="title-font font-medium text-col-pink tracking-widest text-sm
          mb-3">
          Sources
        </h2>
        <nav class="list-none mb-10">
          <li>
            <a
              class="text-white-600 hover:text-white-800"
              href="https://github.com/noandrea/costoflife-rs">
              Library
            </a>
          </li>
          <li>
            <a
              class="text-white-600 hover:text-white-800"
              href="https://github.com/noandrea/costoflife.ui">
              Ui
            </a>
          </li>
        </nav>
      </div>
      <div class="lg:w-1/4 md:w-1/2 w-full px-4">
        <h2
          class="title-font font-medium text-col-pink tracking-widest text-sm
          mb-3">
          Other projects
        </h2>
        <nav class="list-none mb-10">
          <li>
            <a
              class="text-white-600 hover:text-white-800"
              href="https://meetvalis.com">
              VALIS
            </a>
          </li>
        </nav>
      </div>
    </div>
  </div>
  <div class="bg-col-white">
    <div
      class="container mx-auto py-4 flex flex-wrap flex-col sm:flex-row
      text-center">
      <p class="text-gray-500 text-sm sm:text-left">
        © 2021 CostOf.Life —
        <a
          href="https://adgb.me"
          rel="noopener noreferrer"
          class="text-gray-600 ml-1"
          target="_blank">
          @adgb
        </a>
      </p>
      <span
        class="inline-flex sm:ml-auto sm:mt-0 mt-2 justify-center
        sm:justify-start">
        <a class="ml-3 text-gray-500" href="https://twitter.com/noandrea">
          <svg
            fill="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            class="w-5 h-5"
            viewBox="0 0 24 24">
            <path
              d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66
              10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5
              4.5 0 00-.08-.83A7.72 7.72 0 0023 3z" />
          </svg>
        </a>
        <a class="ml-3 text-gray-500" href="https://github.com/noandrea">
          <svg
            fill="currentColor"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24">
            <path
              d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207
              11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729
              1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304
              3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931
              0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0
              1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005
              2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653
              1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0
              4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0
              .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386
              0-6.627-5.373-12-12-12z" />
          </svg>
        </a>
        <a
          class="ml-3 text-gray-500"
          href="https://linkedin.com/in/andreagiacobino/">
          <svg
            fill="currentColor"
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="0"
            class="w-5 h-5"
            viewBox="0 0 24 24">
            <path
              stroke="none"
              d="M16 8a6 6 0 016 6v7h-4v-7a2 2 0 00-2-2 2 2 0 00-2 2v7h-4v-7a6 6
              0 016-6zM2 9h4v12H2z" />
            <circle cx="4" cy="4" r="2" stroke="none" />
          </svg>
        </a>
      </span>
    </div>
  </div>
</footer>
