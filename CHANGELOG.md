# Changelog

All notable changes to EvorBrain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Code Cleanup and Improvements**
  - Removed unused `@mantine/dates` dependency
  - Created proper "Coming Soon" pages for all placeholder routes
  - Added entity list pages (Areas, Goals, Projects, Tasks) with proper routing
  - Cleaned up empty onClick handlers in Sidebar.tsx with proper navigation
  - Command Palette now shows informative notification until implemented

- **Code Quality Analysis** (2025-07-26)
  - Conducted comprehensive code quality audit
  - Identified strengths:
    - 100% type safety with no `any` types
    - Excellent FSD architecture implementation (90% compliance)
    - Well-organized imports and consistent patterns
    - Good React patterns with functional components and hooks
  - Documented areas for improvement:
    - 74 instances of unwrap()/expect() in Rust code need proper error handling
    - Authentication currently stubbed (ProtectedRoute hardcoded to true)
    - No React performance optimizations (React.memo, useMemo, useCallback)
    - Main bundle size at 341KB (target: <250KB)
    - Minimal test coverage across codebase
    - 3 empty workspace packages pending implementation

- **Phase 2: Core Features** (In Progress)
  - Starting implementation of data model for Areas, Goals, Projects, and Tasks
  - Beginning work on file storage system with YAML frontmatter
  - Initial state management setup with Zustand
  - Completed entity TypeScript interfaces (TASK-027):
    - Created dual interface pattern with data-only and methods-enabled interfaces
    - Defined Area, Goal, Project, and Task interfaces with comprehensive properties
    - Added business logic methods for all entity types (query, calculation, validation, relationships)
    - Implemented shared base entity methods for persistence, metadata, and history
    - Created recurrence support for tasks with RecurrenceRule interface
    - Added comprehensive usage examples and documentation
  - Completed Rust entity structs with serialization (TASK-028):
    - Created Entity trait for common fields and behaviors
    - Implemented Area, Goal, Project, and Task structs matching TypeScript interfaces
    - Added RecurrenceRule struct with proper serialization
    - Implemented comprehensive validation methods for all entity types
    - Updated database schema to include all new fields (tags, progress, sort_order, etc.)
    - Fixed all Tauri command handlers to use the new struct fields
    - Added proper date handling and serialization with serde
  - Completed Area entity CRUD operations (TASK-029):
    - Implemented create_area with comprehensive validation (title length, required fields)
    - Added get_area and get_all_areas with proper ordering by sort_order
    - Enhanced update_area to support partial updates including tags and sort_order
    - Implemented delete_area with cascade checking for associated goals
    - Added detailed error handling with user-friendly messages and recovery suggestions
    - Created comprehensive test suite for Area CRUD operations
    - Integrated logging for all Area operations
  - Completed Goal entity CRUD operations (TASK-030):
    - Implemented create_goal with area association validation and automatic sort_order calculation
    - Added get_goal, get_all_goals, and get_goals_by_area with proper ordering
    - Enhanced update_goal to support partial updates for all fields including progress and tags
    - Implemented delete_goal with project cascade checking and search index cleanup
    - Fixed Goal status enum to match TypeScript interface ("not-started", "in-progress", "completed", "abandoned")
    - Added comprehensive validation for progress values (0-100)
    - Created unit tests for Goal CRUD operations with temporary database support
    - Integrated error handling with detailed recovery suggestions

- **Architecture Decision Records (ADRs)** - Added 6 new ADRs based on code quality analysis:
  - ADR-014: State Management Implementation with Zustand
  - ADR-015: Comprehensive Error Handling Strategy
  - ADR-016: React Performance Optimization
  - ADR-017: Authentication System Architecture
  - ADR-018: Testing Strategy Implementation
  - ADR-019: Security Hardening Measures

- **Technical Documentation Updates**
  - Added Code Quality Standards section to PLANNING.md with metrics and targets
  - Documented current technical debt with priority levels
  - Created Performance Optimization Strategy
  - Added Security Architecture Enhancement plan
  - Defined Testing Implementation Plan with coverage targets

## [Phase 1 Complete] - 2025-07-25

### Summary

🎉 **Phase 1: Foundation is complete!** The foundation phase has established a solid technical base for the EvorBrain project. All Phase 1 tasks (TASK-001 through TASK-026) are complete, providing a comprehensive foundation for building the core knowledge management features in Phase 2.

### Added

- **Foundation Phase Complete** 🎉
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
- Basic IPC commands implementation:
  - Created comprehensive Tauri command structure for all entity types
  - Implemented CRUD operations for Areas, Goals, Projects, and Tasks
  - Added entity query commands: get single, get all, get by parent
  - Implemented task filtering commands (by status, upcoming, subtasks)
  - Added proper error handling with AppError type
  - Integrated all commands with SQLite database operations
- Comprehensive error handling system:
  - Created rich error types with error codes and severity levels
  - Implemented user-friendly error messages with recovery suggestions
  - Added structured error serialization for frontend consumption
  - Created error context system with helpful information for users
  - Integrated logging with proper log levels (info, warn, error, critical)
  - Added colorized console logging with timestamps
  - Created error utility functions and extension traits
  - Implemented validation helpers for common scenarios
  - Added error test panel in Settings for developers
  - Enhanced all IPC commands with proper error handling and logging

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
