/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,ts}",
  ],
  theme: {
    extend: {},
    fontSize: {
      // ... other font sizes
      '2xs': '0.7rem', // 6px
    },
  },
  plugins: [],
}