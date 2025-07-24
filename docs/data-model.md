# EvorBrain Data Model Schema

This document describes the comprehensive data model and schema design for EvorBrain, including entity structures, relationships, storage patterns, and full-text search implementation.

## Overview

EvorBrain uses a hierarchical data model to organize knowledge:

```
Life Areas
  └── Goals
      └── Projects
          └── Tasks
```

Each entity is stored as a markdown file with YAML frontmatter containing structured metadata.

## Entity Schemas

### Area (Life Area)

**Purpose**: Represents major life domains (e.g., Health, Career, Relationships)

**File Format**: `areas/[id]-[slug].md`

```yaml
---
id: area_[nanoid]
type: area
title: 'Career Development'
description: 'Professional growth and career advancement'
color: '#FF6B6B'
icon: 'briefcase'
status: active
created_at: 2024-01-01T00:00:00Z
updated_at: 2024-01-01T00:00:00Z
order: 1
tags: ['work', 'professional']
metadata:
  custom_field: 'value'
---
# Career Development

[Markdown content with notes, thoughts, and documentation about this life area]
```

**Fields**:

- `id`: Unique identifier with `area_` prefix
- `type`: Always "area" for areas
- `title`: Display name (required)
- `description`: Brief description
- `color`: Hex color for UI display
- `icon`: Icon identifier for UI
- `status`: active | archived | deleted
- `created_at`: ISO 8601 timestamp
- `updated_at`: ISO 8601 timestamp
- `order`: Sort order for display
- `tags`: Array of string tags
- `metadata`: Object for custom fields

### Goal

**Purpose**: Long-term objectives within a life area

**File Format**: `goals/[id]-[slug].md`

```yaml
---
id: goal_[nanoid]
type: goal
area_id: area_abc123
title: 'Become a Senior Software Engineer'
description: 'Advance technical skills and leadership capabilities'
status: in_progress
priority: high
target_date: 2025-12-31
progress: 45
created_at: 2024-01-01T00:00:00Z
updated_at: 2024-01-01T00:00:00Z
completed_at: null
tags: ['career', 'technical', 'leadership']
metadata:
  milestones:
    - 'Complete advanced TypeScript course'
    - 'Lead a major project'
    - 'Mentor junior developers'
---
# Become a Senior Software Engineer

[Goal details, planning, progress notes, and reflections]
```

**Fields**:

- `id`: Unique identifier with `goal_` prefix
- `type`: Always "goal" for goals
- `area_id`: Reference to parent area (required)
- `title`: Goal title (required)
- `description`: Goal description
- `status`: not_started | in_progress | completed | abandoned
- `priority`: low | medium | high
- `target_date`: Target completion date (ISO 8601)
- `progress`: Percentage (0-100)
- `created_at`: ISO 8601 timestamp
- `updated_at`: ISO 8601 timestamp
- `completed_at`: Completion timestamp (nullable)
- `tags`: Array of string tags
- `metadata`: Custom fields and milestones

### Project

**Purpose**: Concrete initiatives to achieve goals

**File Format**: `projects/[id]-[slug].md`

```yaml
---
id: project_[nanoid]
type: project
goal_id: goal_xyz789
title: 'Build Personal Portfolio Website'
description: 'Create professional portfolio to showcase projects'
status: active
priority: high
start_date: 2024-01-15
due_date: 2024-03-01
progress: 30
created_at: 2024-01-01T00:00:00Z
updated_at: 2024-01-01T00:00:00Z
completed_at: null
tags: ['portfolio', 'web-development', 'career']
metadata:
  estimated_hours: 40
  actual_hours: 12
  deliverables:
    - 'Design mockups'
    - 'Responsive website'
    - 'Project case studies'
---
# Build Personal Portfolio Website

[Project plan, requirements, progress tracking, and notes]
```

**Fields**:

- `id`: Unique identifier with `project_` prefix
- `type`: Always "project" for projects
- `goal_id`: Reference to parent goal (required)
- `title`: Project title (required)
- `description`: Project description
- `status`: planning | active | on_hold | completed | cancelled
- `priority`: low | medium | high
- `start_date`: Project start date
- `due_date`: Project deadline
- `progress`: Percentage (0-100)
- `created_at`: ISO 8601 timestamp
- `updated_at`: ISO 8601 timestamp
- `completed_at`: Completion timestamp (nullable)
- `tags`: Array of string tags
- `metadata`: Custom fields, deliverables, time tracking

### Task

**Purpose**: Actionable items within projects or standalone

**File Format**: `tasks/[id]-[slug].md`

```yaml
---
id: task_[nanoid]
type: task
project_id: project_def456 # Optional - can be null for standalone tasks
title: 'Design homepage wireframe'
description: 'Create low-fidelity wireframe for portfolio homepage'
status: todo
priority: medium
due_date: 2024-01-20T17:00:00Z
estimated_duration: 120 # minutes
actual_duration: null
completed_at: null
recurrence: null # See recurrence section below
created_at: 2024-01-01T00:00:00Z
updated_at: 2024-01-01T00:00:00Z
tags: ['design', 'wireframe', 'ui']
checklist:
  - text: 'Research portfolio examples'
    completed: true
  - text: 'Sketch layout ideas'
    completed: false
  - text: 'Create digital wireframe'
    completed: false
metadata:
  assigned_to: null
  energy_level: 'high' # high | medium | low
  context: '@computer' # GTD context
---
# Design homepage wireframe

[Task details, notes, reference materials, and progress]
```

**Fields**:

- `id`: Unique identifier with `task_` prefix
- `type`: Always "task" for tasks
- `project_id`: Reference to parent project (nullable)
- `title`: Task title (required)
- `description`: Task description
- `status`: todo | in_progress | completed | cancelled
- `priority`: low | medium | high
- `due_date`: Due date and time (ISO 8601)
- `estimated_duration`: Estimated minutes
- `actual_duration`: Actual minutes spent
- `completed_at`: Completion timestamp
- `recurrence`: Recurrence rule (see below)
- `created_at`: ISO 8601 timestamp
- `updated_at`: ISO 8601 timestamp
- `tags`: Array of string tags
- `checklist`: Array of subtask objects
- `metadata`: Custom fields (context, energy, etc.)

### Recurrence Schema

For recurring tasks, the `recurrence` field follows this structure:

```yaml
recurrence:
  rule: 'FREQ=WEEKLY;BYDAY=MO,WE,FR' # RFC 5545 RRULE
  next_due: 2024-01-22T09:00:00Z
  instances_ahead: 3 # Number of future instances to generate
  exceptions: # Dates to skip
    - 2024-02-14
    - 2024-12-25
  modifications: # Instance-specific changes
    - date: 2024-01-29
      changes:
        due_date: 2024-01-30T09:00:00Z
        description: 'Modified for holiday'
```

## Relationships

### Hierarchical Relationships

```
Area (1) ← → (N) Goals
Goal (1) ← → (N) Projects
Project (1) ← → (N) Tasks
```

### Cross-References

Entities can reference each other using markdown links:

```markdown
Related to [[goal_xyz789|Become a Senior Software Engineer]]
Blocked by [[task_abc123|Complete authentication module]]
```

### Backlinks

The system automatically tracks backlinks when entities reference each other, storing them in the SQLite database for quick lookup.

## SQLite Database Schema

### Core Tables

```sql
-- Entity metadata table (mirrors frontmatter)
CREATE TABLE entities (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('area', 'goal', 'project', 'task')),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    parent_id TEXT,
    file_path TEXT NOT NULL UNIQUE,
    frontmatter JSON NOT NULL,  -- Full YAML frontmatter as JSON
    content_hash TEXT NOT NULL,  -- SHA-256 of file content
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    indexed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Relationships table
CREATE TABLE relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,  -- 'parent', 'reference', 'blocks'
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (target_id) REFERENCES entities(id) ON DELETE CASCADE,
    UNIQUE(source_id, target_id, relationship_type)
);

-- Tags table
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Entity-tag junction table
CREATE TABLE entity_tags (
    entity_id TEXT NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (entity_id, tag_id)
);
```

### Full-Text Search Tables (FTS5)

```sql
-- FTS5 virtual table for full-text search
CREATE VIRTUAL TABLE entities_fts USING fts5(
    id UNINDEXED,  -- Entity ID (not searchable)
    title,         -- Weighted heavily in search
    description,   -- Weighted medium
    content,       -- Markdown content (weighted normal)
    tags,          -- Space-separated tags
    type UNINDEXED,  -- For filtering
    status UNINDEXED,  -- For filtering
    parent_id UNINDEXED,  -- For scoped search
    tokenize = 'porter unicode61',  -- Porter stemming with Unicode support
    content_rowid = 'rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER entities_ai AFTER INSERT ON entities BEGIN
    INSERT INTO entities_fts(
        rowid, id, title, description, content, tags, type, status, parent_id
    ) VALUES (
        new.rowid,
        new.id,
        new.title,
        new.description,
        new.content,
        (SELECT GROUP_CONCAT(t.name, ' ')
         FROM entity_tags et
         JOIN tags t ON et.tag_id = t.id
         WHERE et.entity_id = new.id),
        new.type,
        new.status,
        new.parent_id
    );
END;

CREATE TRIGGER entities_au AFTER UPDATE ON entities BEGIN
    UPDATE entities_fts SET
        title = new.title,
        description = new.description,
        content = new.content,
        tags = (SELECT GROUP_CONCAT(t.name, ' ')
                FROM entity_tags et
                JOIN tags t ON et.tag_id = t.id
                WHERE et.entity_id = new.id),
        type = new.type,
        status = new.status,
        parent_id = new.parent_id
    WHERE id = new.id;
END;

CREATE TRIGGER entities_ad AFTER DELETE ON entities BEGIN
    DELETE FROM entities_fts WHERE id = old.id;
END;
```

## Search Implementation

### Search Query Examples

```sql
-- Simple text search across all fields
SELECT id, title, snippet(entities_fts, 2, '<mark>', '</mark>', '...', 30) as excerpt
FROM entities_fts
WHERE entities_fts MATCH 'portfolio'
ORDER BY rank;

-- Search with field-specific weighting
SELECT id, title,
       bm25(entities_fts, 10.0, 5.0, 1.0, 0.5) as score
FROM entities_fts
WHERE entities_fts MATCH 'title:portfolio OR description:website'
ORDER BY score;

-- Complex search with filters
SELECT e.id, e.title, e.type, e.status
FROM entities e
JOIN entities_fts fts ON e.id = fts.id
WHERE fts MATCH 'career development'
  AND e.type = 'goal'
  AND e.status IN ('in_progress', 'not_started')
ORDER BY rank;

-- Search within a specific parent scope
SELECT id, title
FROM entities_fts
WHERE entities_fts MATCH 'meeting'
  AND parent_id = 'project_abc123';
```

### Search Indexing Strategy

1. **Immediate Indexing**: Index on file save for real-time search
2. **Batch Re-indexing**: Periodic full re-index for consistency
3. **Incremental Updates**: Only re-index changed files
4. **Content Extraction**: Strip markdown formatting for cleaner search

## File Storage Structure

```
evorbrain_data/
├── areas/
│   ├── area_1a2b3c-career-development.md
│   └── area_4d5e6f-health-fitness.md
├── goals/
│   ├── goal_7g8h9i-become-senior-engineer.md
│   └── goal_0j1k2l-run-marathon.md
├── projects/
│   ├── project_3m4n5o-portfolio-website.md
│   └── project_6p7q8r-training-plan.md
├── tasks/
│   ├── task_9s0t1u-design-wireframe.md
│   └── task_2v3w4x-morning-run.md
├── attachments/
│   └── [entity_id]/
│       ├── image.png
│       └── document.pdf
└── .evorbrain/
    ├── database.db
    ├── config.json
    └── backups/
```

## Data Integrity & Validation

### Validation Rules

1. **ID Uniqueness**: All IDs must be globally unique
2. **Parent Existence**: Parent IDs must reference existing entities
3. **Type Consistency**: Parent-child relationships must follow hierarchy
4. **Required Fields**: title, type, status, created_at, updated_at
5. **Date Logic**: updated_at >= created_at, completed_at requires completed status

### File-Database Sync

1. **Source of Truth**: Markdown files are the source of truth
2. **Database as Cache**: SQLite serves as queryable cache
3. **Sync on Startup**: Full scan and sync on application start
4. **File Watching**: Real-time sync on file changes
5. **Conflict Resolution**: File always wins in conflicts

## Migration Strategy

### Schema Versioning

Each entity includes a schema version in frontmatter:

```yaml
---
schema_version: 1
# ... other fields
---
```

### Migration Process

1. **Detection**: Check schema_version on file read
2. **Migration**: Apply migrations to bring to current version
3. **Backup**: Create backup before migration
4. **Validation**: Validate migrated data
5. **Update**: Write migrated content back to file

## Performance Considerations

### Indexing Performance

- **Lazy Loading**: Only index files when accessed
- **Batch Operations**: Group multiple index updates
- **Background Processing**: Index in background thread
- **Incremental Updates**: Only re-index changed content

### Query Optimization

- **Index Usage**: Ensure queries use appropriate indexes
- **Pagination**: Limit result sets for large queries
- **Caching**: Cache frequent queries in memory
- **Prepared Statements**: Use prepared statements for repeated queries

## Security Considerations

### Data Protection

1. **File Permissions**: Restrict file access to user only
2. **SQL Injection**: Use parameterized queries exclusively
3. **Path Traversal**: Validate all file paths
4. **Encryption**: Support for encrypted storage (future)

### Privacy

1. **Local Storage**: All data stored locally
2. **No Telemetry**: No data sent to external services
3. **Secure Deletion**: Overwrite deleted files
4. **Backup Encryption**: Optional encryption for backups

## Future Enhancements

### Planned Features

1. **Attachment Indexing**: Index PDF, images with OCR
2. **Graph Database**: Neo4j for relationship queries
3. **Version Control**: Built-in versioning for entities
4. **Sync Protocol**: For multi-device synchronization
5. **Plugin Data**: Extensible schema for plugins

### Schema Evolution

The schema is designed to evolve through:

- Additional optional fields
- New entity types via plugins
- Extended metadata objects
- Backward-compatible changes only
