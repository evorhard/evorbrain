// Application constants
export const APP_CONFIG = {
  NAME: 'EvorBrain',
  VERSION: '0.1.0',
  DESCRIPTION: 'A locally-hosted, hierarchical second brain application',
} as const;

// API configuration
export const API_CONFIG = {
  BASE_URL:
    process.env.NODE_ENV === 'production'
      ? 'https://localhost:3000'
      : 'http://localhost:3000',
  ENDPOINTS: {
    LIFE_AREAS: '/api/life-areas',
    GOALS: '/api/goals',
    PROJECTS: '/api/projects',
    TASKS: '/api/tasks',
    HIERARCHY: '/api/hierarchy',
    EXPORT: '/api/export',
    IMPORT: '/api/import',
    BACKUP: '/api/backup',
  },
  WEBSOCKET_URL:
    process.env.NODE_ENV === 'production'
      ? 'wss://localhost:3000/ws'
      : 'ws://localhost:3000/ws',
} as const;

// Database configuration
export const DB_CONFIG = {
  FILENAME: 'evorbrain.db',
  MIGRATIONS_DIR: 'migrations',
  SEEDS_DIR: 'seeds',
} as const;

// Entity status options
export const ENTITY_STATUS = {
  LIFE_AREA: {
    ACTIVE: 'active',
    ARCHIVED: 'archived',
  },
  GOAL: {
    ACTIVE: 'active',
    COMPLETED: 'completed',
    PAUSED: 'paused',
    CANCELLED: 'cancelled',
  },
  PROJECT: {
    PLANNING: 'planning',
    ACTIVE: 'active',
    COMPLETED: 'completed',
    PAUSED: 'paused',
    CANCELLED: 'cancelled',
  },
  TASK: {
    TODO: 'todo',
    IN_PROGRESS: 'in_progress',
    COMPLETED: 'completed',
    BLOCKED: 'blocked',
  },
} as const;

// Task priority options
export const TASK_PRIORITY = {
  LOW: 'low',
  MEDIUM: 'medium',
  HIGH: 'high',
  URGENT: 'urgent',
} as const;

// Relationship types
export const RELATIONSHIP_TYPES = {
  DEPENDS_ON: 'depends_on',
  BLOCKS: 'blocks',
  RELATES_TO: 'relates_to',
} as const;

// UI configuration
export const UI_CONFIG = {
  COLORS: {
    PRIMARY: '#3b82f6',
    SECONDARY: '#6b7280',
    SUCCESS: '#10b981',
    WARNING: '#f59e0b',
    ERROR: '#ef4444',
    INFO: '#06b6d4',
  },
  BREAKPOINTS: {
    SM: '640px',
    MD: '768px',
    LG: '1024px',
    XL: '1280px',
    '2XL': '1536px',
  },
  ANIMATION: {
    DURATION: {
      FAST: '150ms',
      NORMAL: '300ms',
      SLOW: '500ms',
    },
    EASING: {
      DEFAULT: 'cubic-bezier(0.4, 0, 0.2, 1)',
      IN: 'cubic-bezier(0.4, 0, 1, 1)',
      OUT: 'cubic-bezier(0, 0, 0.2, 1)',
      IN_OUT: 'cubic-bezier(0.4, 0, 0.2, 1)',
    },
  },
} as const;

// Validation constants
export const VALIDATION = {
  MIN_NAME_LENGTH: 1,
  MAX_NAME_LENGTH: 255,
  MAX_DESCRIPTION_LENGTH: 1000,
  MAX_METADATA_KEY_LENGTH: 100,
  MAX_METADATA_VALUE_LENGTH: 1000,
  DEFAULT_PAGE_SIZE: 20,
  MAX_PAGE_SIZE: 100,
} as const;

// WebSocket configuration
export const WS_CONFIG = {
  RECONNECT: {
    MAX_ATTEMPTS: 5,
    INITIAL_DELAY: 1000,
    MAX_DELAY: 30000,
    BACKOFF_FACTOR: 2,
  },
  HEARTBEAT: {
    INTERVAL: 30000,
    TIMEOUT: 5000,
  },
} as const;

// Performance thresholds
export const PERFORMANCE = {
  VIRTUAL_SCROLL_THRESHOLD: 100,
  DEBOUNCE_DELAY: 300,
  THROTTLE_DELAY: 100,
  CACHE_TTL: 5 * 60 * 1000, // 5 minutes
} as const;

// Error codes
export const ERROR_CODES = {
  VALIDATION_ERROR: 'VALIDATION_ERROR',
  NOT_FOUND: 'NOT_FOUND',
  UNAUTHORIZED: 'UNAUTHORIZED',
  FORBIDDEN: 'FORBIDDEN',
  CONFLICT: 'CONFLICT',
  INTERNAL_ERROR: 'INTERNAL_ERROR',
  DATABASE_ERROR: 'DATABASE_ERROR',
  WEBSOCKET_ERROR: 'WEBSOCKET_ERROR',
} as const;
