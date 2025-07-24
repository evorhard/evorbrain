# Changelog

All notable changes to EvorBrain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure and documentation
- Core application architecture using Tauri 2.0
- Monorepo setup with pnpm workspaces
- Basic documentation including README, SECURITY, LICENSE
- Plugin system architecture design
- Feature-Sliced Design (FSD) implementation
- CI/CD pipeline with GitHub Actions:
  - Automated testing for frontend and Rust code
  - Multi-platform build support (Windows, macOS, Linux)
  - Code quality checks (ESLint, Prettier, Clippy)
  - Security auditing with npm audit and cargo audit
  - Release automation with changelog generation
  - Pull request automation and labeling
  - Dependency updates with Dependabot
  - Documentation deployment to GitHub Pages
  - Code coverage reporting
  - Bundle size analysis
  - Lighthouse performance testing
- Tauri permission configuration:
  - File system access permissions with security scopes
  - Dialog and notification permissions
  - Window control permissions
  - Content Security Policy (CSP) configuration
- Development certificate generation and trust scripts
- Comprehensive build scripts:
  - Platform-specific builds (Windows, macOS, Linux)
  - Development and production build modes
  - Build configuration management
  - Rust tooling integration
- Basic Tauri window implementation:
  - Window controls (minimize, maximize, center)
  - IPC communication testing
  - Hot reload functionality
  - App information display
- React frontend with Vite:
  - React 18 with TypeScript in strict mode
  - Vite build tool configuration
  - Hot module replacement (HMR) setup
  - Path aliases and TypeScript references
- Mantine UI v7 integration:
  - Core components and hooks
  - Notifications system
  - Date picker components
  - Forms with validation
  - Theme provider with auto color scheme
- Tailwind CSS configuration:
  - Integration with Mantine components
  - Custom utility classes
  - Dark mode support compatible with Mantine
  - Preflight disabled to prevent conflicts
- Rust backend foundation:
  - Organized src-tauri directory structure with modules
  - Created commands module for Tauri IPC commands
  - Completed database module structure for SQLite integration
  - Implemented SQLite database initialization with all required tables
  - Added database connection management with rusqlite
  - Created and tested basic CRUD operations for database entities
  - Added test command to verify database functionality
- Base layout components:
  - Implemented AppShell layout with Mantine AppShell component
  - Created header with navigation including theme toggle (dark/light mode)
  - Built collapsible sidebar with navigation items and search functionality
  - Added main content area with proper spacing and responsive design
  - Integrated Tabler Icons for consistent UI iconography
  - Applied Feature-Sliced Design (FSD) architecture for component organization
- Routing implementation:
  - Installed and configured React Router v7 with TypeScript support
  - Created route definitions with lazy loading for all main views
  - Implemented protected route component for future authentication
  - Built navigation state management with useNavigation hook
  - Created breadcrumb component for navigation visualization
  - Integrated router with existing sidebar navigation
  - Implemented following pages with basic UI:
    - Daily View page with task tabs (Today, Upcoming, Completed)
    - Calendar View page (placeholder for Phase 3)
    - Hierarchy View page with tree visualization
    - Settings page with appearance, general, data, and plugin tabs
  - Added route utilities for entity navigation and search
  - Updated AppLayout to work with React Router outlet pattern
- Data model and search infrastructure:
  - Comprehensive data model schema documentation
  - SQLite FTS5 (Full-Text Search) implementation
  - Entity storage schema with hierarchical relationships
  - Search indexing and query functionality
  - Test commands for database and FTS5 verification
- File system abstraction layer:
  - Created comprehensive file system abstraction with async operations
  - Implemented file read/write operations with atomic writes
  - Added file watching and change detection with debouncing
  - Created file path validation and security checks
  - Implemented utilities for filename sanitization and generation
  - Added support for backup files and metadata operations
  - Integrated with Tauri's security model to prevent path traversal

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

## [1.0.0] - Phase 6 Release

### Added

- **Core Features**
  - Hierarchical workflow system (Life Areas → Goals → Projects → Tasks)
  - Daily/Upcoming view with status filtering
  - Calendar view with drag-and-drop scheduling
  - Hierarchical tree view
  - Task management with recurring tasks support
  - Tag-based categorization system
  - Full-text search across all content
  - Backlinks and relationships between items

- **Data Management**
  - Markdown file storage with YAML frontmatter
  - SQLite database with FTS5 for indexing
  - Automatic backup system
  - Data import/export functionality

- **User Interface**
  - Modern React-based UI with Mantine components
  - Dark/Light theme support
  - Keyboard shortcuts for common actions
  - Command palette for quick navigation
  - Customizable layouts

- **Plugin System**
  - VSCode-style extension host pattern
  - Built-in plugins: calendar-view, graph-view, task-recurring
  - Plugin marketplace integration
  - Developer-friendly plugin API

- **Performance**
  - Sub-2-second application startup
  - Fast file operations (<100ms)
  - Efficient search across 10,000+ files
  - 60 FPS UI interactions
  - Memory usage optimization (<200MB)

### Security

- Local-only data storage
- Sandboxed plugin execution
- Secure IPC communication
- Optional plugin signature verification

## [0.9.0-beta] - Phase 5 Beta

### Added

- Beta release for community testing
- Core functionality complete
- Basic plugin system
- Documentation and tutorials

### Changed

- UI refinements based on alpha feedback
- Performance optimizations

### Fixed

- Various bugs from alpha testing
- Memory leaks in long-running sessions

## [0.5.0-alpha] - Phase 2 Alpha

### Added

- Alpha release with core features
- Basic task management
- File storage system
- Simple UI implementation

### Known Issues

- Performance optimization needed
- Limited keyboard shortcuts
- No plugin support yet

## Links

- [Unreleased]: https://github.com/evorhard/evorbrain/compare/v1.0.0...HEAD
- [1.0.0]: https://github.com/evorhard/evorbrain/compare/v0.9.0-beta...v1.0.0
- [0.9.0-beta]: https://github.com/evorhard/evorbrain/compare/v0.5.0-alpha...v0.9.0-beta
- [0.5.0-alpha]: https://github.com/evorhard/evorbrain/releases/tag/v0.5.0-alpha

---

For more details on the versioning and release process, see [CONTRIBUTING.md](CONTRIBUTING.md).
