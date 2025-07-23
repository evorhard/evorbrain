/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [],
  darkMode: ['class', '[data-mantine-color-scheme="dark"]'],
  theme: {
    extend: {
      colors: {
        // Brand colors - keeping minimal set for future use
        brand: {
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Consolas', 'Monaco', 'monospace'],
      },
    },
  },
  plugins: [],
  // Ensure Tailwind utilities don't conflict with Mantine styles
  corePlugins: {
    preflight: false, // Disable Tailwind's base reset since Mantine has its own
  },
};