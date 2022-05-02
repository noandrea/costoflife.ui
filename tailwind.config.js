// tailwind.config.js
const production = !process.env.ROLLUP_WATCH; // or some other env var like NODE_ENV
module.exports = {
  future: { // for tailwind 2.0 compat
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
  plugins: [
    // for tailwind UI users only
    // require('@tailwindcss/ui'),
    // other plugins here
  ],
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
    // may also want to include base index.html
  ],
  theme: {
    extend: {
      colors: {
        'costoflife-blue': '#586736',
        'costoflife-light-blue': '#1DC7DA',
        'col-red': "#f14136",
        'col-pink': "#f7bba4",
        'col-blue': "#053044",
        'col-white': "#f8f0dc"
      }
    }
  }
};