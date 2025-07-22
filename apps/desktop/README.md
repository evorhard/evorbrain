# @evorbrain/desktop

The main desktop application for EvorBrain, built with Tauri, React, and TypeScript.

## Architecture

This package contains:
- React frontend application
- Tauri configuration and integration
- Desktop-specific UI components and views
- State management setup
- Routing configuration

## Development

```bash
# Run in development mode
pnpm dev

# Run with Tauri (desktop window)
pnpm tauri:dev

# Build for production
pnpm build
pnpm tauri:build
```

## Structure

```
apps/desktop/
├── src/                # React application source
│   ├── app/           # App initialization and providers
│   ├── pages/         # Page components
│   ├── widgets/       # Complex UI blocks
│   ├── features/      # Feature-specific code
│   └── main.tsx       # Entry point
├── src-tauri/         # Tauri backend (Rust)
├── public/            # Static assets
└── index.html         # HTML template
```

## Dependencies

- `@evorbrain/core` - Core business logic
- `@evorbrain/ui` - Shared UI components
- `@tauri-apps/api` - Tauri API bindings
- `react` & `react-dom` - UI framework
- `zustand` - State management
- `jotai` - Atomic state management
- `@mantine/core` - UI component library