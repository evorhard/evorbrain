# @evorbrain/plugin-api

Plugin API definitions and interfaces for EvorBrain plugin development.

## Overview

This package provides:
- TypeScript interfaces for plugin development
- Plugin lifecycle hooks
- Event system definitions
- Permission system types
- Helper utilities for plugin authors

## Architecture

```
packages/plugin-api/
├── src/
│   ├── interfaces/    # Core plugin interfaces
│   ├── hooks/         # Lifecycle hook definitions
│   ├── events/        # Event system types
│   ├── permissions/   # Permission system
│   ├── helpers/       # Helper utilities
│   └── index.ts       # Public API exports
├── docs/              # Generated API documentation
└── tests/             # API tests
```

## Plugin Structure

```typescript
import { Plugin, PluginContext } from '@evorbrain/plugin-api';

export default class MyPlugin implements Plugin {
  id = 'my-plugin';
  name = 'My Plugin';
  version = '1.0.0';
  
  async activate(context: PluginContext): Promise<void> {
    // Plugin initialization
  }
  
  async deactivate(): Promise<void> {
    // Cleanup
  }
}
```

## Key Concepts

### Plugin Manifest

Every plugin must have a `plugin.json` manifest:

```json
{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "1.0.0",
  "main": "./dist/index.js",
  "permissions": ["read", "write"],
  "contributes": {
    "commands": [],
    "views": [],
    "settings": []
  }
}
```

### Lifecycle Hooks

- `activate()` - Called when plugin is loaded
- `deactivate()` - Called when plugin is unloaded
- `onReady()` - Called when app is fully initialized
- `onSettingsChange()` - Called when settings change

### Permissions

Plugins must declare required permissions:
- `read` - Read access to data
- `write` - Write access to data
- `ui` - Modify UI elements
- `network` - Network access
- `system` - System-level operations

## Development

```bash
# Build the API
pnpm build

# Generate documentation
pnpm docs

# Run tests
pnpm test
```

## For Plugin Authors

See the [Plugin Development Guide](../../docs/plugin-development.md) for detailed instructions on creating EvorBrain plugins.