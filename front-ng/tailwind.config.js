/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,ts}",
  ],
  theme: {
    extend: {
      fontSize: {
      
      '2xs': '0.7rem', 
      'xs': '0.8rem'
    }, 
    },
    
  },
  plugins: [],
}