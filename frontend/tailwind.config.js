// tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"], // Shadcn adds this
  content: ['./src/**/*.{html,js,svelte,ts}',
      './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'], // For Flowbite
  theme: {
    extend: {},
  },
  plugins: [require('flowbite/plugin')],
};

