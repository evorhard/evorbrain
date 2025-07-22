# @evorbrain/core

Core business logic, data models, and utilities for EvorBrain.

## Overview

This package contains all the core functionality that is shared between different parts of the application:
- Entity data models (Area, Goal, Project, Task)
- Business logic and validation rules
- Data transformation utilities
- Type definitions and schemas
- Constants and enums

## Architecture

```
packages/core/
├── src/
│   ├── entities/      # Data models for each entity type
│   ├── utils/         # Utility functions and helpers
│   ├── types/         # TypeScript type definitions
│   ├── schemas/       # Zod validation schemas
│   ├── constants/     # Shared constants and enums
│   └── index.ts       # Public API exports
└── tests/             # Unit tests
```

## Usage

```typescript
import { Task, TaskStatus, createTask } from '@evorbrain/core';

const task = createTask({
  title: 'My Task',
  status: TaskStatus.NotStarted,
  // ... other properties
});
```

## Development

```bash
# Build the package
pnpm build

# Watch mode for development
pnpm dev

# Run tests
pnpm test

# Type checking
pnpm typecheck
```

## Design Principles

1. **Pure Functions**: All functions should be pure with no side effects
2. **Type Safety**: Full TypeScript coverage with strict mode
3. **Validation**: Use Zod schemas for runtime validation
4. **Immutability**: Return new objects rather than mutating
5. **Testability**: Keep functions small and focused