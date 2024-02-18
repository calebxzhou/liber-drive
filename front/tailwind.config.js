/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      width: {
        '1/16': '6.25%',
        '1/24': '4.166666666666667%'
      }

    },
  },
  plugins: [],
}
 