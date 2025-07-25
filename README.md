<div align="center">
  <h1>
    <br>
    🧠 EvorBrain
    <br>
  </h1>
  
  <h3>An offline-first, open-source personal knowledge management system</h3>
  
  <p>
    <img src="https://img.shields.io/badge/Status-Under_Construction-orange?style=for-the-badge" alt="Status: Under Construction" />
    <img src="https://img.shields.io/badge/Phase-2_Core_Features-blue?style=for-the-badge" alt="Phase 2: Core Features" />
    <img src="https://img.shields.io/badge/License-Apache_2.0-green?style=for-the-badge" alt="License: Apache 2.0" />
  </p>
  
  <p>
    <img src="https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat-square&logo=tauri&logoColor=white" alt="Tauri" />
    <img src="https://img.shields.io/badge/React-18-61DAFB?style=flat-square&logo=react&logoColor=black" alt="React" />
    <img src="https://img.shields.io/badge/TypeScript-5.3-3178C6?style=flat-square&logo=typescript&logoColor=white" alt="TypeScript" />
    <img src="https://img.shields.io/badge/SQLite-FTS5-003B57?style=flat-square&logo=sqlite&logoColor=white" alt="SQLite" />
  </p>
  
  <h4>
    <a href="#-planned-features">Features</a>
    <span> · </span>
    <a href="#-development">Development</a>
    <span> · </span>
    <a href="#-roadmap">Roadmap</a>
    <span> · </span>
    <a href="#-contributing">Contributing</a>
  </h4>
  
  <br>
  
  <div style="background-color: #fff3cd; border: 2px solid #ffeaa7; border-radius: 8px; padding: 16px; margin: 16px auto; max-width: 600px;">
    <h3 style="margin: 0 0 8px 0;">⚠️ Project Status: Under Construction</h3>
    <p style="margin: 0;">
      This project is currently in early development. Features and documentation are being actively developed.
      See <a href="TASKS.md">TASKS.md</a> for detailed development roadmap and <a href="ROADMAP.md">ROADMAP.md</a> for planned milestones.
    </p>
  </div>
</div>

## 🌟 Overview

> **Your knowledge, your control, your privacy.**

EvorBrain will be a powerful, customizable personal knowledge management system designed for offline-first operation. Built with modern web technologies and packaged as a native desktop application using Tauri, it will combine the best features of tools like Notion and Obsidian while maintaining complete data ownership and privacy.

✨ **Why EvorBrain?**

- 🔒 **100% Offline** - Your data never leaves your device
- 📝 **Markdown-based** - Future-proof and portable
- ⚡ **Lightning Fast** - Native performance with Rust backend
- 🎨 **Beautiful UI** - Modern, intuitive interface
- 🔧 **Extensible** - Plugin system for custom workflows
- 🌍 **Open Source** - Transparent and community-driven

## 📋 Planned Features

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

## 📊 Current Status

<div align="center">
  <table>
    <tr>
      <td align="center">✅</td>
      <td><strong>Phase 1: Foundation</strong></td>
      <td>Complete</td>
    </tr>
    <tr>
      <td align="center">🚧</td>
      <td><strong>Phase 2: Core Features</strong></td>
      <td>In Progress</td>
    </tr>
    <tr>
      <td align="center">🚧</td>
      <td><strong>Data Model Implementation</strong></td>
      <td>In Progress</td>
    </tr>
    <tr>
      <td align="center">💾</td>
      <td><strong>File Storage System</strong></td>
      <td>⏳ Planned</td>
    </tr>
    <tr>
      <td align="center">📊</td>
      <td><strong>State Management</strong></td>
      <td>⏳ Planned</td>
    </tr>
    <tr>
      <td align="center">🎨</td>
      <td><strong>Basic UI Components</strong></td>
      <td>⏳ Planned</td>
    </tr>
    <tr>
      <td align="center">🔍</td>
      <td><strong>Search Implementation</strong></td>
      <td>⏳ Planned</td>
    </tr>
  </table>
</div>

**🚧 Note:** The application is not yet ready for use. See the development section below to run from source.

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

## 🛠 Development

### 🏗 Architecture

EvorBrain uses a pnpm monorepo structure with:

- **apps/desktop**: Tauri desktop application (React + TypeScript)
- **packages/core**: Core business logic and data models
- **packages/ui**: Shared UI components and design system
- **packages/plugin-api**: Plugin system APIs and interfaces
- **plugins/**: Built-in plugins (coming in Phase 4)
- **docs/**: Technical documentation including [data model schema](docs/data-model.md)

### 💻 Tech Stack

- **Desktop Framework**: Tauri 2.0
- **Frontend**: React 18 + TypeScript
- **State Management**: Zustand + Jotai
- **UI Components**: Mantine v7 + Tailwind CSS
- **Build Tool**: Vite
- **Database**: SQLite with FTS5

### 🚀 Development Setup

#### Prerequisites:

<table>
  <tr>
    <td>📦 Node.js</td>
    <td>18+ (LTS)</td>
    <td><a href="https://nodejs.org/">Download</a></td>
  </tr>
  <tr>
    <td>📦 pnpm</td>
    <td>8+</td>
    <td><code>npm install -g pnpm</code></td>
  </tr>
  <tr>
    <td>🦀 Rust</td>
    <td>1.70+</td>
    <td><a href="https://rustup.rs/">Install</a></td>
  </tr>
  <tr>
    <td>🔧 Tauri CLI</td>
    <td>2.0</td>
    <td><code>cargo install tauri-cli</code></td>
  </tr>
</table>

```bash
# Clone the repository
git clone https://github.com/evorhard/evorbrain.git
cd evorbrain

# Install dependencies
pnpm install

# Development commands:
pnpm dev                       # Run all packages in dev mode
pnpm --filter desktop dev      # Run desktop app only
pnpm --filter desktop tauri:dev # Run desktop app with Tauri
pnpm build                     # Build all packages
pnpm test                      # Run all tests
pnpm lint                      # Lint all packages
pnpm typecheck                 # Type check all packages
```

### Plugin Development

Plugin system is planned for Phase 4 of development. Documentation will be available once the core system is implemented.

## 🗺 Roadmap

Development is organized into six phases:

| Phase                       | Focus                                             | Status         |
| --------------------------- | ------------------------------------------------- | -------------- |
| **1️⃣ Foundation**           | Project setup, Tauri integration, basic structure | ✅ Complete    |
| **2️⃣ Core Features**        | Data model, file storage, basic UI                | 🚧 In Progress |
| **3️⃣ Enhanced Features**    | Calendar, drag-and-drop, advanced UI              | ⏳ Planned     |
| **4️⃣ Plugin System**        | Extensibility and third-party plugins             | ⏳ Planned     |
| **5️⃣ Performance & Polish** | Optimization, testing, security                   | ⏳ Planned     |
| **6️⃣ Release**              | Distribution, documentation, community            | ⏳ Planned     |

See [ROADMAP.md](ROADMAP.md) for detailed milestones and [TASKS.md](TASKS.md) for granular task breakdown.

## 🤝 Contributing

Contributions are welcome once the basic architecture is established!

### How You Can Help:

🎯 **Core Features Phase** (Current)

- Test early alpha builds and report issues
- Provide feedback on UI/UX design
- Help with documentation and guides

🧪 **Testing Phase** (Phase 3+)

- Test early builds and report bugs
- Provide UX/UI feedback
- Help with cross-platform testing

🔌 **Plugin Phase** (Phase 4+)

- Develop plugins for specific workflows
- Create themes and customizations
- Build integrations with other tools

📚 **Always Welcome**

- Documentation improvements
- Translation help (Phase 5+)
- Community support and engagement

> 📋 Contributing guidelines are being developed. Please open an issue or discussion for major changes.

## 📄 License

EvorBrain is licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

This project is being built with these amazing open-source technologies:

- [Tauri](https://tauri.app/) - Secure, lightweight desktop framework
- [React](https://react.dev/) - UI library
- [Mantine](https://mantine.dev/) - React components library
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [Zustand](https://github.com/pmndrs/zustand) - State management
- [SQLite](https://sqlite.org/) - Local database

## 💬 Get Involved

<div align="center">
  
| Platform | Purpose | Link |
|----------|---------|------|
| 💬 **Discussions** | Ask questions, share ideas | [Join the conversation](https://github.com/evorhard/evorbrain/discussions) |
| 🐛 **Issues** | Report bugs, request features | [View issues](https://github.com/evorhard/evorbrain/issues) |
| 📋 **Project Board** | Track development progress | [See progress](https://github.com/evorhard/evorbrain/projects) |
| ⭐ **Star** | Show your support | [Star this repo](https://github.com/evorhard/evorbrain) |

</div>
