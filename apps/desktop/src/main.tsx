import React from 'react';

import { MantineProvider } from '@mantine/core';
// import { DatesProvider } from '@mantine/dates';
import { Notifications } from '@mantine/notifications';
import { createRoot } from 'react-dom/client';

import { App } from './App';

// Import Mantine CSS
import '@mantine/core/styles.css';
// import '@mantine/dates/styles.css';
import '@mantine/notifications/styles.css';

// Import our custom styles
import './index.css';

const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error('Root element not found');
}

createRoot(rootElement).render(
  <React.StrictMode>
    <MantineProvider defaultColorScheme="auto">
      <Notifications />
      <App />
    </MantineProvider>
  </React.StrictMode>,
);
