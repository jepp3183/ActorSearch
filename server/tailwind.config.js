/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/**/*.{html, js}"],
  theme: {
    extend: {
      colors: {
        "movie":"#264653",
        "background":"#2a9d8f",
        "buttonhover":"#f4a261",
        "button":"#e76f51"
      }
    },
  },
  plugins: [],
}

