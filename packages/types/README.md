# @evorbrain/types

Shared TypeScript type definitions for the EvorBrain monorepo.

## Overview

This package contains:
- Common type definitions used across all packages
- Shared interfaces and enums
- Utility types
- Global type augmentations

## Structure

```
packages/types/
└── src/
    ├── common/        # Common types used everywhere
    ├── entities/      # Entity-related types
    ├── api/           # API and IPC types
    ├── ui/            # UI-specific types
    ├── utils/         # Utility type helpers
    └── index.d.ts     # Main export file
```

## Usage

```typescript
import type { EntityStatus, ID, Timestamp } from '@evorbrain/types';

// Use the imported types
const status: EntityStatus = 'active';
const id: ID = 'abc123';
```

## Adding New Types

1. Create type definitions in the appropriate subdirectory
2. Export them from the subdirectory's index file
3. Re-export from the main `index.d.ts`
4. Ensure all types are properly documented with JSDoc

## Type Guidelines

- Use `interface` for object shapes that might be extended
- Use `type` for unions, intersections, and utility types
- Always export types with explicit `export type` syntax
- Document all types with JSDoc comments
- Avoid using `any` - use `unknown` or generic types instead