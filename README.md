# EvorBrain

[![Build Status](https://img.shields.io/badge/build-development-yellow.svg)](https://github.com/user/evorbrain)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/user/evorbrain)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2011-blue.svg)](https://github.com/user/evorbrain)

A powerful digital garden and second brain application that combines flexible knowledge management with structured project organization to help you capture ideas, manage tasks, and build your personal knowledge ecosystem.

---

## 📖 Table of Contents

- [What does this do?](#-what-does-this-do)
- [Key Features](#-key-features)
- [Technology Stack](#-technology-stack)
- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
- [Getting Started](#-getting-started)
- [Project Structure](#-project-structure)
- [Development](#-development)
- [Documentation](#-documentation)
- [Contributing](#-contributing)
- [License](#-license)
- [Support](#-support)

---

## 🤔 What does this do?

EvorBrain is designed for knowledge workers, students, entrepreneurs, and anyone who wants to organize their digital life while building a connected knowledge base. Think of it as a combination of:

- **Personal Project Manager**: Organize your life using a clear hierarchy (Life Areas → Goals → Projects → Tasks)
- **Digital Garden**: Create interconnected notes and ideas that grow over time through bi-directional linking
- **Second Brain**: Capture, process, and retrieve information when you need it most

Unlike traditional project management tools that focus purely on tasks, or note-taking apps that lack structure, EvorBrain bridges both worlds. You can manage concrete projects with deadlines while also nurturing long-term ideas and knowledge that doesn't fit into a specific project timeline.

**Perfect for**:

- Managing both personal and professional projects
- Building a personal knowledge base that evolves with you
- Connecting ideas across different areas of your life
- Working entirely offline while maintaining full control of your data

---

## ✨ Key Features

### 🏗️ Hierarchical Project Management

- **Life Areas**: Organize by major life domains (Work, Health, Learning, etc.)
- **Goals**: Define long-term objectives within each life area
- **Projects**: Break goals into actionable projects with clear outcomes
- **Tasks**: Track specific actions with deadlines, priorities, and dependencies

### 🌱 Digital Garden Capabilities

- **Bi-directional Linking**: Connect notes and ideas across your entire knowledge base
- **Dynamic Content**: Notes, project documentation, and insights that evolve over time
- **Knowledge Discovery**: Find unexpected connections between seemingly unrelated concepts
- **Flexible Organization**: Information doesn't need to fit rigid folder structures

### 🔒 Privacy & Control

- **100% Local-First**: All data stays on your device - no cloud dependency
- **Offline-Capable**: Full functionality without internet connection
- **Data Ownership**: Export your data anytime in standard formats
- **Privacy-Focused**: No tracking, no analytics, no external data sharing

### ⚡ Performance & Usability

- **Fast Search**: Instant full-text search across all your content
- **Modern Interface**: Clean, responsive design optimized for Windows 11
- **Keyboard-Driven**: Extensive shortcuts for power users
- **Real-time Sync**: Changes reflect immediately across all views

---

## 🛠️ Technology Stack

EvorBrain is built with modern web technologies wrapped in a native desktop application:

- **Frontend Framework**: React 19 with TypeScript for type-safe, component-based UI
- **Desktop Runtime**: Tauri 2.0 for native performance with web technology flexibility
- **UI Components**: Chakra UI v3 for consistent, accessible design system
- **State Management**: Valtio for reactive, proxy-based state management
- **Database**: RxDB with IndexedDB for offline-first data persistence
- **Build System**: Vite for fast development and optimized production builds

_For detailed technical architecture, see [PLANNING.md](PLANNING.md)_

---

## 📋 Prerequisites

### System Requirements

- **Operating System**: Windows 11 (primary target) or Windows 10 (22H2+)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 500MB for application, additional space for your data
- **Display**: 1366x768 minimum resolution, 1920x1080 recommended

### Development Requirements

- **Node.js**: Version 18.17.0+ (20.x LTS recommended) ([Download](https://nodejs.org/))
- **Rust**: Latest stable version ([Install via rustup](https://rustup.rs/))
- **Git**: For version control ([Download](https://git-scm.com/))
- **VS Code**: Recommended editor with TypeScript support

### Verification Commands

```bash
# Check Node.js version
node --version  # Should be v20.0.0+

# Check Rust installation
rustc --version

# Check Git installation
git --version
```

---

## 🚀 Installation

### Option 1: Download Release (Recommended)

1. Go to the [Releases page](https://github.com/user/evorbrain/releases)
2. Download the latest `.msi` installer for Windows
3. Run the installer and follow the setup wizard
4. Launch EvorBrain from Start Menu or Desktop shortcut

### Option 2: Build from Source

```bash
# 1. Clone the repository
git clone https://github.com/user/evorbrain.git
cd evorbrain

# 2. Install dependencies
npm install

# 3. Build the application
npm run tauri build

# 4. Install the generated .msi file
# Located in: src-tauri/target/release/bundle/msi/
```

### Option 3: Development Setup

```bash
# 1. Clone and enter directory
git clone https://github.com/user/evorbrain.git
cd evorbrain

# 2. Install dependencies
npm install

# 3. Start development server
npm run tauri dev
```

---

## 🎯 Getting Started

### First Launch

1. **Welcome Screen**: Follow the initial setup wizard
2. **Data Location**: Choose where to store your EvorBrain data
3. **Sample Data**: Optionally import sample projects to explore features

### Basic Workflow

1. **Create Life Areas**: Start with 3-5 major areas of your life

   - Examples: Work, Health, Learning, Personal Projects, Relationships

2. **Define Goals**: Add 1-3 goals per life area

   - Make them specific and measurable when possible

3. **Add Projects**: Break goals into concrete projects

   - Each project should have a clear outcome and timeline

4. **Create Tasks**: Add actionable tasks to projects
   - Use deadlines, priorities, and dependencies as needed

### Knowledge Management

1. **Create Notes**: Capture ideas, insights, and information
2. **Link Concepts**: Use `[[double brackets]]` to link related ideas
3. **Build Context**: Connect notes to relevant projects and tasks
4. **Review Regularly**: Use the dashboard to see connections and patterns

### Keyboard Shortcuts

- `Ctrl + N`: Quick note creation
- `Ctrl + P`: Command palette
- `Ctrl + K`: Global search
- `Ctrl + /`: Toggle sidebar
- `F11`: Toggle fullscreen

---

## 📁 Project Structure

```
evorbrain/
├── src/                    # React frontend source code
│   ├── components/         # Reusable UI components
│   ├── pages/             # Main application views
│   ├── hooks/             # Custom React hooks
│   ├── stores/            # Valtio state stores
│   ├── utils/             # Utility functions
│   └── types/             # TypeScript type definitions
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/               # Rust source code
│   ├── tauri.conf.json    # Tauri configuration
│   └── Cargo.toml         # Rust dependencies
├── public/                # Static assets
├── docs/                  # Documentation
├── PLANNING.md            # Detailed technical planning
├── TASKS.md              # Development task tracking
└── package.json          # Node.js dependencies
```

---

## 🔧 Development

### Available Scripts

```bash
# Development
npm run tauri dev          # Start development server with hot reload
npm run dev               # Frontend-only development

# Building
npm run tauri build       # Build production application
npm run build            # Build frontend only

# Testing
npm run test             # Run unit tests
npm run test:e2e         # Run end-to-end tests
npm run lint             # Run ESLint
npm run type-check       # TypeScript type checking

# Database
npm run db:migrate       # Run database migrations
npm run db:seed          # Seed development data
```

### Development Workflow

1. **Start Development**: `npm run tauri dev`
2. **Make Changes**: Edit files in `src/` or `src-tauri/src/`
3. **Test Changes**: Application hot-reloads automatically
4. **Run Tests**: `npm run test` before committing
5. **Check Types**: `npm run type-check` for TypeScript validation

### Debugging

- **Frontend**: Use browser DevTools (F12 in development)
- **Backend**: Rust logs appear in terminal running `tauri dev`
- **Database**: Use built-in database inspector (Ctrl + Shift + D)

---

## 📚 Documentation

### Core Documentation

- **[PLANNING.md](PLANNING.md)**: Complete technical architecture, decisions, and roadmap
- **[TASKS.md](TASKS.md)**: Current development status and task tracking
- **API Documentation**: Generated docs available at `/docs` (development mode)

### External Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React 19 Documentation](https://react.dev/)
- [Chakra UI v3 Guide](https://chakra-ui.com/)
- [RxDB Documentation](https://rxdb.info/)

---

## 🤝 Contributing

We welcome contributions from the community! Here's how to get involved:

### Development Workflow

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature-name`
3. **Make** your changes following our coding standards
4. **Test** your changes thoroughly
5. **Commit** with descriptive messages
6. **Push** to your fork: `git push origin feature-name`
7. **Submit** a Pull Request

### Coding Standards

- Follow TypeScript best practices with strict type checking
- Use ESLint configuration for consistent code style
- Write unit tests for new functionality
- Update documentation for user-facing changes
- Follow conventional commit format for commit messages

### Areas for Contribution

- **Features**: New functionality based on roadmap in PLANNING.md
- **Bug Fixes**: Issues reported in GitHub Issues
- **Documentation**: Improvements to guides and API docs
- **Testing**: Expand test coverage and add integration tests
- **Performance**: Optimization and efficiency improvements

### Getting Help

- Check existing [Issues](https://github.com/user/evorbrain/issues) and [Discussions](https://github.com/user/evorbrain/discussions)
- Read through [PLANNING.md](PLANNING.md) for technical context
- Join our community discussions for questions and ideas

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### What this means:

- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Private use allowed
- ❌ No liability or warranty provided

---

## 💬 Support

### Getting Help

- **Documentation**: Check [PLANNING.md](PLANNING.md) for technical details
- **Issues**: Report bugs or request features via [GitHub Issues](https://github.com/user/evorbrain/issues)
- **Discussions**: Ask questions in [GitHub Discussions](https://github.com/user/evorbrain/discussions)
- **Email**: Contact the maintainer at [email@example.com](mailto:email@example.com)

### Current Status

This project is in active development. Check [TASKS.md](TASKS.md) for current progress and upcoming features.

---

_Built with ❤️ for knowledge workers who value privacy, structure, and the power of connected thinking._
