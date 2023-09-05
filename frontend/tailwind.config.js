/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  //darkMode: 'media',
  plugins: [
    require("daisyui")
  ],
  daisyui: {
    themes: [
        //"luxury"
        "synthwave"
    ],
  },
};