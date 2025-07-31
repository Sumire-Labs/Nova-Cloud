/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      animation: {
        'aurora': 'aurora 60s linear infinite',
      },
      keyframes: {
        aurora: {
          from: {
            background-position: '0% 50%',
          },
          to: {
            background-position: '200% 50%',
          },
        },
      },
    },
  },
  plugins: [],
}
