# @evorbrain/ui

Shared UI components and design system for EvorBrain.

## Overview

This package contains:
- Reusable React components
- Design tokens and theme configuration
- Shared styles and utilities
- Component documentation via Storybook

## Architecture

```
packages/ui/
├── src/
│   ├── components/    # Reusable UI components
│   ├── hooks/         # Custom React hooks
│   ├── styles/        # Global styles and utilities
│   ├── theme/         # Mantine theme configuration
│   ├── utils/         # UI utility functions
│   └── index.ts       # Public API exports
├── stories/           # Storybook stories
└── tests/             # Component tests
```

## Component Categories

- **Layout**: AppShell, Sidebar, Header, Footer
- **Forms**: Input, Select, DatePicker, TextArea
- **Feedback**: Alert, Toast, Loading, Empty
- **Display**: Card, Badge, Avatar, Progress
- **Navigation**: Tabs, Breadcrumbs, Menu
- **Overlays**: Modal, Drawer, Popover, Tooltip

## Usage

```typescript
import { Button, Card, Badge } from '@evorbrain/ui';

function MyComponent() {
  return (
    <Card>
      <Badge color="blue">Active</Badge>
      <Button variant="filled">Click me</Button>
    </Card>
  );
}
```

## Development

```bash
# Build the package
pnpm build

# Watch mode for development
pnpm dev

# Run Storybook
pnpm storybook

# Run tests
pnpm test
```

## Design Principles

1. **Accessibility First**: All components follow WCAG 2.1 AA standards
2. **Composability**: Components should be small and composable
3. **Consistency**: Follow Mantine design patterns
4. **Performance**: Use React.memo and optimize re-renders
5. **Documentation**: Every component has Storybook stories