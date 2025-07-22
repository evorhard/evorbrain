# EvorBrain

<div align="center">
  <!-- <img src="logo.png" alt="EvorBrain Logo" width="200"/> -->
  <p><strong>An offline-first, open-source personal knowledge management system</strong></p>
  
  > ⚠️ **Project Status: Under Construction** ⚠️
  > 
  > This project is currently in early development. Features and documentation are being actively developed.
  > See [TASKS.md](TASKS.md) for detailed development roadmap and [ROADMAP.md](ROADMAP.md) for planned milestones.
  
  <p>
    <a href="#planned-features">Planned Features</a> •
    <a href="#development">Development</a> •
    <a href="#contributing">Contributing</a> •
    <a href="#roadmap">Roadmap</a>
  </p>
</div>

## Overview

EvorBrain will be a powerful, customizable personal knowledge management system designed for offline-first operation. Built with modern web technologies and packaged as a native desktop application using Tauri, it will combine the best features of tools like Notion and Obsidian while maintaining complete data ownership and privacy.

## Planned Features

### 🎯 Core Functionality

- **Hierarchical Workflow System**
  - Life Areas for top-level organization
  - Goals linked to Life Areas
  - Projects with definite endpoints
  - Tasks with flexible organization
  - Support for standalone tasks and subtasks

- **Multiple Views**
  - Daily/Upcoming view with status filtering
  - Calendar view with drag-and-drop scheduling
  - Hierarchical tree view of your entire system
  - Quick search across all content

- **Task Management**
  - Recurring tasks (daily, weekly, monthly)
  - Tag-based categorization
  - Status tracking (not started, in progress, completed)
  - Due dates with calendar integration
  - Backlinks and relationships between items

### 🔒 Privacy & Control

- **100% Offline**: All data stored locally as markdown files
- **Open Source**: Complete transparency and customizability
- **Data Portability**: Your data in standard markdown format with YAML frontmatter
- **No Vendor Lock-in**: Export and use your data anywhere

### 🚀 Performance

- Application startup: < 2 seconds
- File operations: < 100ms
- Search across 10,000 files: < 500ms
- Smooth 60 FPS UI interactions
- Memory usage: < 200MB typical

### 🎨 Customization

- Plugin system for extending functionality
- Customizable themes and layouts
- Keyboard shortcuts
- Template system for common workflows

## Current Status

🚧 **In Development** - The application is not yet ready for use. See the development section below to run from source.

### Planned System Requirements

- **Windows**: Windows 10/11 (64-bit)
- **macOS**: macOS 10.15+ (planned)
- **Linux**: Ubuntu 20.04+ or equivalent (planned)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 500MB for application, additional space for your data

### Future Installation

Once development is complete, installation will be available via:
- Direct download from GitHub Releases
- Package managers (Windows Store, Mac App Store, Linux repositories)
- Portable versions for all platforms

## Development

### Architecture

EvorBrain uses a monorepo structure with:
- **apps/desktop**: Tauri desktop application
- **packages/core**: Core business logic
- **packages/ui**: Shared UI components
- **packages/plugin-api**: Plugin system APIs
- **plugins/**: Built-in plugins

### Tech Stack

- **Desktop Framework**: Tauri 2.0
- **Frontend**: React 18 + TypeScript
- **State Management**: Zustand + Jotai
- **UI Components**: Mantine v7 + Tailwind CSS
- **Build Tool**: Vite
- **Database**: SQLite with FTS5

### Development Setup

Prerequisites:
- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Tauri CLI

```bash
# Clone the repository
git clone https://github.com/evorhard/evorbrain.git
cd evorbrain

# Install dependencies (when package.json is created)
pnpm install

# Future development commands:
# pnpm --filter desktop dev    # Run desktop app in dev mode
# pnpm test                    # Run tests
# pnpm lint                    # Lint code
# pnpm build                   # Build all packages
```

### Plugin Development

Plugin system is planned for Phase 4 of development. Documentation will be available once the core system is implemented.

## Roadmap

Development is organized into phases:

- **Phase 1**: Foundation (Project setup, Tauri integration, basic structure)
- **Phase 2**: Core Features (Data model, file storage, basic UI)
- **Phase 3**: Enhanced Features (Calendar, drag-and-drop, advanced UI)
- **Phase 4**: Plugin System (Extensibility and third-party plugins)
- **Phase 5**: Performance & Polish (Optimization, testing, security)
- **Phase 6**: Release (Distribution, documentation, community)

See [ROADMAP.md](ROADMAP.md) for detailed milestones and [TASKS.md](TASKS.md) for granular task breakdown.

## Contributing

Contributions are welcome once the basic architecture is established! Early contributors can help with:

- Architecture feedback and suggestions
- Testing and bug reports (once testable builds are available)
- Documentation improvements
- Plugin development (Phase 4+)

Formal contributing guidelines will be established during Phase 1 development.

## License

EvorBrain is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

This project will be built with amazing open-source technologies:

- [Tauri](https://tauri.app/) - Secure, lightweight desktop framework
- [React](https://react.dev/) - UI library
- [Mantine](https://mantine.dev/) - React components library
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [Zustand](https://github.com/pmndrs/zustand) - State management
- [SQLite](https://sqlite.org/) - Local database

## Get Involved

- 💬 [GitHub Discussions](https://github.com/evorhard/evorbrain/discussions) - Ask questions, share ideas
- 🐛 [Issue Tracker](https://github.com/evorhard/evorbrain/issues) - Report bugs, request features
- 📋 [Project Board](https://github.com/evorhard/evorbrain/projects) - Track development progress

---