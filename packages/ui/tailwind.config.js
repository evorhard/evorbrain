/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Custom colors will be added here to match Mantine theme
      },
    },
  },
  plugins: [],
  // Important to avoid conflicts with Mantine styles
  corePlugins: {
    preflight: false,
  },
}