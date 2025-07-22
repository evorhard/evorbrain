// @evorbrain/plugin-api - Public API exports

// Core interfaces (to be implemented)
// export * from './interfaces/index.js';

// Hook definitions (to be implemented)
// export * from './hooks/index.js';

// Event system (to be implemented)
// export * from './events/index.js';

// Permission system (to be implemented)
// export * from './permissions/index.js';

// Helper utilities (to be implemented)
// export * from './helpers/index.js';

// Placeholder export to prevent empty module error
export const version = '0.1.0';

// Basic plugin interface placeholder
export interface Plugin {
  id: string;
  name: string;
  version: string;
  description?: string;
  author?: string;
  
  activate?(): Promise<void>;
  deactivate?(): Promise<void>;
}