# EvorBrain 🧠

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0--alpha-red.svg)](https://github.com/user/evorbrain)
[![Build Status](https://img.shields.io/badge/build-pending-orange.svg)](https://github.com/user/evorbrain/actions)
[![Development Phase](https://img.shields.io/badge/phase-Phase%201.1%20Complete-brightgreen.svg)](TASKS.md)

**A locally-hosted, hierarchical second brain application for personal knowledge and task management**

---

## 📑 Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Technical Architecture](#-technical-architecture)
- [Getting Started](#-getting-started)
- [Project Status & Roadmap](#-project-status--roadmap)
- [Development](#-development)
- [Documentation](#-documentation)
- [Contributing](#-contributing)
- [License](#-license)
- [Support](#-support)

---

## 🎯 Overview

EvorBrain is a comprehensive second brain application designed to help individuals organize their thoughts, goals, and tasks using a natural four-level hierarchy system. Unlike traditional task managers, EvorBrain provides a structured approach to personal knowledge management that scales from high-level life planning down to daily task execution.

### The Four-Level Hierarchy

```
🏠 Life Areas (Health, Career, Relationships, etc.)
    └── 🎯 Goals (Specific objectives within each area)
        └── 📋 Projects (Concrete initiatives to achieve goals)
            └── ✅ Tasks (Individual actionable items)
```

### Target Audience

- **Knowledge Workers**: Professionals managing complex projects across multiple domains
- **Students**: Organizing academic and personal development goals
- **Entrepreneurs**: Balancing multiple business initiatives and personal objectives
- **Anyone**: Seeking a structured approach to personal organization and productivity

### Key Differentiators

- **Hierarchical Structure**: Natural organization that mirrors how we think about life and work
- **Local-First**: Complete data ownership with no cloud dependencies
- **Real-Time Sync**: Changes appear instantly across all browser tabs
- **Cross-Hierarchy Intelligence**: Find connections and relationships across all levels
- **Performance-Focused**: Optimized to handle 10,000+ tasks without performance degradation
- **Future-Ready Architecture**: Designed for extensibility and advanced features

---

## ✨ Features

### 🚀 Core MVP Features

#### ✅ Phase 1.1 - Foundation Complete

- [x] **Project Foundation**: Complete monorepo structure with TypeScript
- [x] **Backend Infrastructure**: Bun runtime with Hono.js framework and SQLite database
- [x] **Frontend Foundation**: React 18 with TypeScript, Vite, and Tailwind CSS
- [x] **Real-Time Communication**: WebSocket integration for instant updates
- [x] **Development Environment**: ESLint, Prettier, and git hooks configured

#### 🚧 In Development (Phases 1.2+)

- [ ] **Hierarchical Organization**: Four-level structure (Life Areas → Goals → Projects → Tasks)
- [ ] **Complete CRUD Operations**: Create, read, update, and delete items at all levels
- [ ] **Status Management**: Comprehensive workflow states for all entity types
- [ ] **Cross-Hierarchy Querying**: Find and filter items across all levels
- [ ] **Drag-and-Drop Reordering**: Intuitive organization with persistent sorting
- [ ] **Data Export/Import**: JSON and CSV formats for data portability
- [ ] **Performance Optimization**: Handles large datasets efficiently
- [ ] **Responsive Design**: Works seamlessly on desktop and mobile devices
- [ ] **Dark/Light Themes**: Customizable interface preferences

### 🔮 Planned Features (Post-MVP)

- [ ] **Note-Taking Integration**: Rich markdown editor with entity linking
- [ ] **File Attachments**: Local file storage with search and versioning
- [ ] **Advanced Search**: Full-text search with SQLite FTS5
- [ ] **Knowledge Graphs**: Visual relationship mapping and insights
- [ ] **Multi-User Support**: Collaboration features with role-based access
- [ ] **Mobile App**: Native mobile applications for iOS and Android
- [ ] **Calendar Integration**: Sync due dates with external calendar systems
- [ ] **Backup & Recovery**: Automated backup with point-in-time restoration
- [ ] **Plugin System**: Extensible architecture for custom functionality
- [ ] **API Access**: RESTful API for third-party integrations

---

## 🏗️ Technical Architecture

### Technology Stack

#### Backend

- **Runtime**: [Bun 1.0+](https://bun.sh) - TypeScript-native runtime with excellent performance
- **Web Framework**: [Hono.js](https://hono.dev) - Lightweight, fast web framework
- **Database**: [SQLite](https://www.sqlite.org) with [Better-SQLite3](https://github.com/JoshuaWise/better-sqlite3)
- **Real-Time**: Native WebSocket support in Bun
- **Validation**: [Zod](https://zod.dev) for runtime type safety

#### Frontend

- **Framework**: [React 18](https://react.dev) with TypeScript
- **Build Tool**: [Vite](https://vitejs.dev) - Fast development and building
- **Styling**: [Tailwind CSS](https://tailwindcss.com) - Utility-first CSS framework
- **State Management**: [Zustand](https://zustand-demo.pmnd.rs) - Lightweight state management
- **Data Fetching**: [TanStack Query](https://tanstack.com/query) - Powerful data synchronization
- **UI Components**: [Shadcn/ui](https://ui.shadcn.com) - Beautiful, accessible components

### Architecture Diagram

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │    Backend      │    │   Database      │
│                 │    │                 │    │                 │
│ React + TypeScript   │ Bun + Hono.js   │    │     SQLite      │
│ Zustand Store   │◄──►│ WebSocket API   │◄──►│ Better-SQLite3  │
│ TanStack Query  │    │ RESTful Routes  │    │ Hierarchical    │
│ Shadcn/ui       │    │ Zod Validation  │    │ Schema Design   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Database Design

The application uses a carefully designed SQLite schema optimized for hierarchical data:

- **life_areas**: Top-level life domains (Health, Career, etc.)
- **goals**: Specific objectives within life areas
- **projects**: Concrete initiatives to achieve goals
- **tasks**: Individual actionable items
- **relationships**: Cross-hierarchy connections
- **metadata**: Extensible key-value storage

### Performance Specifications

- **Startup Time**: < 2 seconds application load
- **API Response**: < 100ms average response time
- **Large Datasets**: Smooth handling of 10,000+ tasks
- **Real-Time Updates**: < 50ms update propagation
- **Memory Usage**: Stable during extended sessions

---

## 🚀 Getting Started

### Prerequisites

- **Bun Runtime**: Version 1.0 or higher ([Install Bun](https://bun.sh))
- **Operating System**: Windows, macOS, or Linux
- **Memory**: Minimum 4GB RAM recommended
- **Storage**: 100MB for application + data storage space
- **Browser**: Modern browser with WebSocket support

### Installation & Development Setup

```bash
# Clone the repository
git clone https://github.com/user/evorbrain.git
cd evorbrain

# Install dependencies for all packages
bun install

# Start the backend server (runs on http://localhost:8080)
cd backend
bun --watch src/index.ts

# In a new terminal, start the frontend development server (runs on http://localhost:5173)
cd frontend
bun run dev

# Or use the root package.json scripts:
bun run backend:dev   # Start backend server
bun run frontend:dev  # Start frontend server
```

### Development Workflow

1. **Backend Server**: Runs on `http://localhost:8080` with hot reload
2. **Frontend Server**: Runs on `http://localhost:5173` with Vite hot reload
3. **Database**: SQLite database file at `backend/evorbrain.db`
4. **Real-time Updates**: WebSocket connection automatically established
5. **Code Quality**: ESLint and Prettier run automatically via git hooks

### Current Implementation Status

**✅ Phase 1.1 Complete** (June 29, 2025):

- Complete monorepo structure with frontend/backend/shared packages
- Bun backend server with Hono.js framework and SQLite database
- React frontend with TypeScript, Vite, and Tailwind CSS
- WebSocket real-time communication established
- Development workflow with ESLint, Prettier, and git hooks

**🚧 Next: Phase 1.2** - Database Design & Schema Implementation

### Tech Stack Implemented

**Backend:**

- [Bun](https://bun.sh) - TypeScript runtime and package manager
- [Hono.js](https://hono.dev) - Lightweight web framework
- [SQLite](https://www.sqlite.org) - Local database
- WebSocket - Real-time communication
- [Zod](https://zod.dev) - Schema validation

**Frontend:**

- [React 18](https://react.dev) with TypeScript
- [Vite](https://vitejs.dev) - Fast build tool
- [Tailwind CSS](https://tailwindcss.com) - Utility-first styling
- [React Router](https://reactrouter.com) - Client-side routing

---

## 📊 Project Status & Roadmap

### Current Status: **Phase 1.1 Complete - Foundation Established**

EvorBrain has successfully completed Phase 1.1 of implementation with the core foundation now established. Both backend and frontend servers are running and the development environment is fully operational.

### Development Timeline

#### 🏗️ Phase 1: Foundation (Weeks 1-2)

**Status**: 16.7% Complete (1 of 6 tasks) • **Focus**: Core Infrastructure

- [x] **1.1** Project Setup & Environment Configuration ✅ **COMPLETED** (June 29, 2025)
- [ ] **1.2** Database Design & Schema Implementation ⭐ **NEXT PRIORITY**
- [ ] **1.3** Basic Backend API Structure
- [ ] **1.4** Frontend Foundation & Basic Components
- [ ] **1.5** Basic CRUD Operations
- [ ] **1.6** Initial WebSocket Integration

#### 🎯 Phase 2: Core Features (Weeks 3-4)

**Status**: 0% Complete • **Focus**: Essential Functionality

- [ ] Complete Hierarchy CRUD Operations
- [ ] Status Management & Workflows
- [ ] Real-time Updates Implementation
- [ ] Cross-hierarchy Querying
- [ ] Data Export/Import Functionality
- [ ] Basic Error Handling & Validation

#### ✨ Phase 3: Polish & Performance (Weeks 5-6)

**Status**: 0% Complete • **Focus**: Production Readiness

- [ ] UI/UX Improvements with v0 Integration
- [ ] Performance Optimization
- [ ] Comprehensive Error Handling
- [ ] Testing Implementation
- [ ] Backup & Recovery System
- [ ] Documentation & Deployment Guide

#### 🔮 Phase 4: Extensions (Post-MVP)

**Status**: 0% Complete • **Focus**: Advanced Features

- [ ] Note-taking Capabilities
- [ ] File Attachments System
- [ ] Advanced Search Implementation
- [ ] Knowledge Graph Features
- [ ] Multi-user Authentication
- [ ] Mobile Responsiveness

### Progress Tracking

For detailed progress tracking and task status, see:

- **[TASKS.md](TASKS.md)**: Complete implementation roadmap with task breakdown
- **[PLANNING.md](PLANNING.md)**: Comprehensive architectural blueprint

---

## 🛠️ Development

### Project Structure

```
evorbrain/
├── frontend/                 # React frontend application
│   ├── src/
│   │   ├── components/      # Reusable UI components
│   │   ├── hooks/           # Custom React hooks
│   │   ├── stores/          # Zustand state management
│   │   ├── types/           # TypeScript definitions
│   │   ├── utils/           # Utility functions
│   │   └── pages/           # Application pages
│   └── package.json
├── backend/                  # Bun backend API
│   ├── src/
│   │   ├── routes/          # API route handlers
│   │   ├── models/          # Database models
│   │   ├── services/        # Business logic
│   │   ├── middleware/      # Request middleware
│   │   └── websocket/       # Real-time handlers
│   └── package.json
├── shared/                   # Shared TypeScript types
├── docs/                     # Project documentation
├── scripts/                  # Build and utility scripts
└── README.md                # This file
```

### Development Setup

_Instructions will be available after Phase 1 implementation_

### Testing Strategy

- **Unit Tests**: Comprehensive coverage for business logic
- **Integration Tests**: API endpoint and database testing
- **Component Tests**: React component behavior verification
- **End-to-End Tests**: Critical user flow validation
- **Performance Tests**: Load testing and optimization verification

### Code Quality Standards

- **TypeScript Strict Mode**: Zero `any` types in production
- **Test Coverage**: Minimum 80% for critical paths
- **Code Review**: All changes require review
- **Automated Linting**: ESLint + Prettier enforcement
- **Git Hooks**: Pre-commit quality checks

---

## 📚 Documentation

### Available Documentation

- **[PLANNING.md](PLANNING.md)**: Complete architectural blueprint and technical specifications
- **[TASKS.md](TASKS.md)**: Detailed implementation roadmap with progress tracking

### Future Documentation

_Will be available as development progresses:_

- **API Documentation**: Complete REST API reference
- **User Guide**: Step-by-step usage instructions with screenshots
- **Development Guide**: Setup and contribution instructions
- **Deployment Guide**: Production deployment instructions
- **Troubleshooting**: Common issues and solutions

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The MIT License allows for:

- ✅ Commercial use
- ✅ Modification and distribution
- ✅ Private use
- ✅ Patent use

---

## 📋 Current Status Summary

**Project Phase**: Phase 1 Foundation (In Progress)
**Development Status**: Phase 1.1 Complete - Foundation Established
**Last Update**: June 29, 2025 at 11:12 PM CET
**Next Priority**: Phase 1.2 - Database Design & Schema Implementation

**Active Servers:**

- Backend: Running at `http://localhost:8080` (Bun + Hono.js + SQLite)
- Frontend: Running at `http://localhost:5173` (React + TypeScript + Vite + Tailwind)
- WebSocket: Real-time communication established

For detailed task progress and implementation roadmap, see [TASKS.md](TASKS.md).

---

> **Generated by**: Claude 4 (Anthropic) - AI-Assisted Documentation Creation  
> **Generated on**: December 29, 2025  
> **Purpose**: Professional project documentation for EvorBrain second brain application

---
