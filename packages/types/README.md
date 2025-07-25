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

### Basic Types

```typescript
import type { EntityStatus, ID, Timestamp } from '@evorbrain/types';

// Use the imported types
const status: EntityStatus = 'active';
const id: ID = 'abc123';
```

### Entity Interfaces

```typescript
import type { Area, Goal, Project, Task } from '@evorbrain/types';

// Basic entity interfaces (data only)
const area: Area = {
  id: 'area_123',
  type: 'area',
  title: 'Health & Fitness',
  description: 'Physical and mental well-being',
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
  sortOrder: 1
};
```

### Entity Interfaces with Methods

```typescript
import type { 
  AreaWithMethods, 
  GoalWithMethods, 
  ProjectWithMethods, 
  TaskWithMethods 
} from '@evorbrain/types';

// Entity interfaces with business logic methods
async function workWithArea(area: AreaWithMethods) {
  const goals = await area.getGoals();
  const progress = area.calculateProgress();
  
  if (area.canBeDeleted()) {
    await area.delete();
  }
}

async function workWithTask(task: TaskWithMethods) {
  if (task.isOverdue()) {
    await task.setPriority('urgent');
  }
  
  await task.start();
  await task.startTimer();
  
  // ... do work ...
  
  await task.stopTimer();
  await task.complete();
  
  if (task.isRecurring()) {
    await task.createNextInstance();
  }
}
```

### Common Methods Available

All entities with methods extend the `BaseEntityMethods` interface:

- **Persistence**: `save()`, `delete()`, `reload()`
- **Metadata**: `addTag()`, `removeTag()`, `hasTag()`
- **Export**: `toMarkdown()`, `toJSON()`, `clone()`
- **History**: `getHistory()`, `revertTo()`

Each entity type also has specific methods for their domain logic. See the [examples](./examples/entity-usage.ts) for detailed usage.

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