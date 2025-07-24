# EvorBrain Roadmap

This document outlines the development roadmap for EvorBrain, organized into phases with clear milestones and timelines.

## Overview

EvorBrain development follows an iterative approach with regular releases:
- **Alpha**: Core functionality (Phase 2)
- **Beta**: Feature-complete with community testing (Phase 5)
- **1.0 Release**: Production-ready (Phase 6)
- **Post-1.0**: Continuous improvement and new features

## Phase 1: Foundation

### Goals
Establish project structure and core architecture

### Milestones
- [x] Project initialization and documentation
- [x] Monorepo setup with pnpm workspaces
- [x] Tauri application scaffold
- [x] Basic Tauri window and IPC communication
- [x] Basic React + TypeScript frontend (completed)
- [x] Mantine UI and Tailwind CSS integration
- [x] React Router v7 integration with lazy loading
- 🚧 SQLite integration with Rust backend (in progress)
- [ ] File system operations for markdown storage

### Deliverables
- Working development environment
- Basic desktop application shell
- File read/write capabilities

## Phase 2: Core Features

### Goals
Implement fundamental knowledge management features

### Milestones
- [ ] Hierarchical data model (Areas → Goals → Projects → Tasks)
- [ ] CRUD operations for all entity types
- [ ] Markdown file storage with YAML frontmatter
- [ ] Basic UI components with Mantine
- [ ] State management with Zustand
- [ ] Search functionality with SQLite FTS5

### Deliverables
- Alpha release (v0.5.0-alpha)
- Core functionality working end-to-end
- Basic user documentation

## Phase 3: Enhanced Features

### Goals
Add advanced features and polish user experience

### Milestones
- [ ] Calendar view with FullCalendar integration
- [ ] Drag-and-drop support with @dnd-kit
- [ ] Recurring tasks implementation
- [ ] Tag system and filtering
- [ ] Backlinks and relationships
- [ ] Keyboard shortcuts
- [ ] Command palette
- [ ] Theme support (dark/light)

### Deliverables
- Beta release (v0.9.0-beta)
- Feature-complete application
- Community beta testing program

## Phase 4: Plugin System

### Goals
Implement extensibility through plugins

### Milestones
- [ ] Plugin API design and implementation
- [ ] Extension host pattern (VSCode-style)
- [ ] Plugin manifest system
- [ ] Built-in plugins:
  - [ ] Calendar view enhancements
  - [ ] Graph visualization
  - [ ] Advanced recurring tasks
- [ ] Plugin development documentation
- [ ] Plugin marketplace infrastructure

### Deliverables
- Plugin SDK
- Example plugins
- Developer documentation

## Phase 5: Performance & Polish

### Goals
Optimize performance and prepare for 1.0 release

### Milestones
- [ ] Performance optimization (<2s startup, <100ms operations)
- [ ] Memory usage optimization (<200MB typical)
- [ ] UI/UX refinements based on beta feedback
- [ ] Comprehensive testing suite
- [ ] Security audit
- [ ] Documentation completion
- [ ] Installation/distribution pipeline

### Deliverables
- Release Candidate (v1.0.0-rc)
- Performance benchmarks
- Security assessment report

## Phase 6: 1.0 Release

### Goals
Production-ready release

### Milestones
- [ ] Bug fixes from RC testing
- [ ] Final documentation review
- [ ] Marketing website
- [ ] Release automation
- [ ] Community launch

### Deliverables
- Version 1.0.0
- Comprehensive documentation
- Marketing materials
- Community resources

## Post-1.0 Roadmap

### Version 1.1 - Phase 7
- [ ] macOS support
- [ ] Linux support
- [ ] Advanced search operators
- [ ] Batch operations
- [ ] Performance monitoring

### Version 1.2 - Phase 8
- [ ] Habit tracking module
- [ ] Basic data visualization
- [ ] Template system
- [ ] Quick capture
- [ ] Mobile companion app planning

### Version 1.3 - Phase 9
- [ ] Health metrics tracking
- [ ] Weight and measurement logging
- [ ] Progress charts
- [ ] Goal visualization
- [ ] Export capabilities

### Version 2.0 - Phase 10
- [ ] Web version
- [ ] Sync capabilities (opt-in)
- [ ] Collaboration features
- [ ] Advanced plugin marketplace
- [ ] AI assistant integration

## Long-term Vision

### Advanced Features
- Reading list management
- Research tools
- Citation management
- Advanced graph visualization
- Natural language processing
- Voice input
- Mobile applications
- Team collaboration

### Platform Expansion
- iOS/Android native apps
- Browser extensions
- API for third-party integrations
- Cloud sync service (self-hostable)
- Enterprise features

## Success Metrics

### Technical Metrics
- Performance targets met (startup, operations, search)
- Memory usage within limits
- Cross-platform compatibility
- Plugin ecosystem growth

### Community Metrics
- Active users
- GitHub stars/forks
- Plugin submissions
- Community contributions
- Documentation quality

### Business Metrics
- Adoption rate
- User retention
- Support ticket volume
- Feature request patterns

## Risk Mitigation

### Technical Risks
- **Tauri stability**: Regular testing, fallback plans
- **Performance issues**: Continuous profiling, optimization
- **Cross-platform bugs**: Extensive testing matrix

### Market Risks
- **Competition**: Focus on unique features, community
- **User adoption**: Marketing, documentation, onboarding

### Resource Risks
- **Development time**: Realistic timelines, buffer included
- **Community contributions**: Clear guidelines, mentorship

## Community Involvement

### How to Contribute
- Feature requests via GitHub Issues
- Beta testing program
- Plugin development
- Documentation improvements
- Bug reports and fixes
- Translations

### Communication Channels
- GitHub Discussions
- Discord server
- Monthly development updates
- Quarterly roadmap reviews

---

This roadmap is a living document and will be updated based on community feedback and development progress. Join the [discussions](https://github.com/yourusername/evorbrain/discussions) to help shape the future of EvorBrain!