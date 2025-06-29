// eslint.config.js
import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import reactHooks from 'eslint-plugin-react-hooks';
import prettier from 'eslint-config-prettier';

export default tseslint.config(
  // Apply ESLint recommended rules
  js.configs.recommended,

  // Apply TypeScript ESLint recommended rules
  ...tseslint.configs.recommended,

  // Global configuration for all TypeScript/JavaScript files
  {
    files: ['**/*.{js,mjs,cjs,ts,tsx}'],
    languageOptions: {
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
    rules: {
      // TypeScript specific rules
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-inferrable-types': 'off',

      // General code quality
      'no-console': 'warn',
      'no-debugger': 'error',
      'prefer-const': 'error',
      'no-var': 'error',
    },
  },

  // Frontend-specific configuration (React/JSX)
  {
    files: ['frontend/**/*.{ts,tsx,js,jsx}'],
    plugins: {
      'react-hooks': reactHooks,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,

      // React specific rules
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',

      // Allow console in development
      'no-console': 'warn',
    },
    languageOptions: {
      parserOptions: {
        ecmaFeatures: {
          jsx: true,
        },
      },
    },
  },

  // Backend-specific configuration
  {
    files: ['backend/**/*.{ts,js}'],
    rules: {
      // Backend can use console for logging
      'no-console': 'off',

      // Stricter TypeScript rules for backend
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/explicit-function-return-type': 'warn',
    },
  },

  // Shared package configuration
  {
    files: ['shared/**/*.{ts,js}'],
    rules: {
      // Stricter rules for shared code
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/explicit-function-return-type': 'warn',
      'no-console': 'error', // No console in shared utilities
    },
  },

  // Test files configuration
  {
    files: [
      '**/*.{test,spec}.{ts,tsx,js,jsx}',
      '**/tests/**/*.{ts,tsx,js,jsx}',
    ],
    rules: {
      // More relaxed rules for tests
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      'no-console': 'off',
    },
  },

  // Configuration files
  {
    files: [
      '*.config.{js,ts,mjs}',
      '.*rc.{js,ts}',
      'vite.config.ts',
      'tailwind.config.js',
    ],
    rules: {
      // Allow require in config files
      '@typescript-eslint/no-var-requires': 'off',
      'no-console': 'off',
    },
  },

  // Ignore patterns
  {
    ignores: [
      '**/node_modules/**',
      '**/dist/**',
      '**/build/**',
      '**/*.d.ts',
      '**/coverage/**',
      '**/.next/**',
      '**/out/**',
      '**/public/**',
      '**/.husky/_/**',
      '**/bun.lock*',
      '**/.vite/**',
      '**/vite/**',
    ],
  },

  // Prettier integration (must be last to override conflicting rules)
  prettier
);
