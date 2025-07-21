# EvorBrain

<div align="center">
  <img src="logo.png" alt="EvorBrain Logo" width="200"/>
  <p><strong>An offline-first, open-source personal knowledge management system</strong></p>
  <p>
    <a href="#features">Features</a> •
    <a href="#installation">Installation</a> •
    <a href="#usage">Usage</a> •
    <a href="#development">Development</a> •
    <a href="#contributing">Contributing</a>
  </p>
</div>

## Overview

EvorBrain is a powerful, customizable personal knowledge management system designed for offline-first operation. Built with modern web technologies and packaged as a native desktop application using Tauri, it combines the best features of tools like Notion and Obsidian while maintaining complete data ownership and privacy.

## Features

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

## Installation

### System Requirements

- **Windows**: Windows 10/11 (64-bit)
- **macOS**: macOS 10.15+ (coming soon)
- **Linux**: Ubuntu 20.04+ or equivalent (coming soon)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 500MB for application, additional space for your data

### Download

Download the latest release from the [Releases](https://github.com/yourusername/evorbrain/releases) page:

1. Download `EvorBrain-Setup-x.x.x.exe` for Windows
2. Run the installer
3. Follow the installation wizard

### Build from Source

Prerequisites:
- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Tauri CLI

```bash
# Clone the repository
git clone https://github.com/yourusername/evorbrain.git
cd evorbrain

# Install dependencies
pnpm install

# Build desktop app
pnpm --filter desktop build

# Or run in development mode
pnpm --filter desktop dev
```

## Usage

### Getting Started

1. **Create Your First Life Area**
   - Click "New Area" in the sidebar
   - Name it (e.g., "Work", "Personal", "Health")
   - Add a description and icon

2. **Add Goals**
   - Within a Life Area, click "Add Goal"
   - Define long-term objectives
   - Link them to your Life Area

3. **Create Projects**
   - Click "New Project" 
   - Set a clear endpoint and timeline
   - Associate with relevant Goals

4. **Manage Tasks**
   - Add tasks within projects or as standalone items
   - Set due dates and recurrence patterns
   - Tag for cross-cutting organization

### Keyboard Shortcuts

- `Ctrl/Cmd + N`: New task
- `Ctrl/Cmd + P`: Quick search
- `Ctrl/Cmd + K`: Command palette
- `Ctrl/Cmd + \`: Toggle sidebar
- `Ctrl/Cmd + 1-3`: Switch between views

### Data Storage

Your data is stored in:
- **Windows**: `%APPDATA%/evorbrain/data/`
- **macOS**: `~/Library/Application Support/evorbrain/data/`
- **Linux**: `~/.config/evorbrain/data/`

Each item is a markdown file with YAML frontmatter:

```markdown
---
id: unique-id-here
type: task
title: Example Task
status: in-progress
tags: [important, work]
parent: project-id
created: 2025-01-20T10:00:00
modified: 2025-01-20T14:30:00
due: 2025-01-25T10:00:00
---

# Example Task

Task description and notes go here...
```

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

### Development Commands

```bash
# Install dependencies
pnpm install

# Run desktop app in dev mode
pnpm --filter desktop dev

# Run tests
pnpm test

# Lint code
pnpm lint

# Build all packages
pnpm build
```

### Plugin Development

Create a new plugin:

```bash
pnpm create-plugin my-plugin
```

See the [Plugin Development Guide](docs/plugin-development.md) for details.

## Contributing

I welcome contributions! Please see the [Contributing Guide](CONTRIBUTING.md) for details on:

- Code of Conduct
- Development setup
- Submitting pull requests
- Reporting issues

### Contributors

<a href="https://github.com/yourusername/evorbrain/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=yourusername/evorbrain" />
</a>

## License

EvorBrain is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- UI powered by [Mantine](https://mantine.dev/) and [Tailwind CSS](https://tailwindcss.com/)
- Icons from [Tabler Icons](https://tabler-icons.io/)

## Support

- 📖 [Documentation](https://evorbrain.dev/docs)
- 💬 [Discussions](https://github.com/yourusername/evorbrain/discussions)
- 🐛 [Issue Tracker](https://github.com/yourusername/evorbrain/issues)
- 📧 Email: support@evorbrain.dev

---