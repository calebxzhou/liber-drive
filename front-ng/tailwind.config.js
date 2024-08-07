/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,ts}",
  ],
  theme: {
    extend: {
      padding:{
        '1/5': '0.05rem'
      },
      fontSize: {
      '3xs': '0.5rem', 
      '2xs': '0.7rem', 
      'xs': '0.8rem'
    }, 
    },
    
  },
  plugins: [],
}