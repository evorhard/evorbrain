const sharedConfig = require('../config/tailwind/tailwind.config.js');

/** @type {import('tailwindcss').Config} */
module.exports = {
  ...sharedConfig,
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  // Important to avoid conflicts with Mantine styles
  corePlugins: {
    preflight: false,
  },
};