import { Database } from 'bun:sqlite';
import { join } from 'path';

let db: Database | null = null;

export function initializeDatabase(): Promise<void> {
  return new Promise((resolve, reject) => {
    try {
      // Create database in the project root
      const dbPath = join(process.cwd(), 'evorbrain.db');
      db = new Database(dbPath);

      // Enable foreign keys
      db.exec('PRAGMA foreign_keys = ON');

      // Create tables
      createTables();

      console.log('✅ Database initialized successfully');
      resolve();
    } catch (error) {
      console.error('❌ Database initialization failed:', error);
      reject(error);
    }
  });
}

export function getDatabase(): Database {
  if (!db) {
    throw new Error(
      'Database not initialized. Call initializeDatabase() first.'
    );
  }
  return db;
}

function createTables(): void {
  if (!db) return;

  // Life Areas table
  db.exec(`
    CREATE TABLE IF NOT EXISTS life_areas (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      color TEXT,
      status TEXT NOT NULL DEFAULT 'active',
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
  `);

  // Goals table
  db.exec(`
    CREATE TABLE IF NOT EXISTS goals (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      life_area_id TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'active',
      priority TEXT NOT NULL DEFAULT 'medium',
      target_date DATETIME,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (life_area_id) REFERENCES life_areas(id) ON DELETE CASCADE
    )
  `);

  // Projects table
  db.exec(`
    CREATE TABLE IF NOT EXISTS projects (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      goal_id TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'active',
      priority TEXT NOT NULL DEFAULT 'medium',
      start_date DATETIME,
      target_date DATETIME,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE
    )
  `);

  // Tasks table
  db.exec(`
    CREATE TABLE IF NOT EXISTS tasks (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      project_id TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'todo',
      priority TEXT NOT NULL DEFAULT 'medium',
      due_date DATETIME,
      completed_at DATETIME,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
    )
  `);

  // Create indexes for better performance
  db.exec(`
    CREATE INDEX IF NOT EXISTS idx_goals_life_area_id ON goals(life_area_id);
    CREATE INDEX IF NOT EXISTS idx_projects_goal_id ON projects(goal_id);
    CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
    CREATE INDEX IF NOT EXISTS idx_life_areas_status ON life_areas(status);
    CREATE INDEX IF NOT EXISTS idx_goals_status ON goals(status);
    CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
    CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
  `);
}

export function closeDatabase(): void {
  if (db) {
    db.close();
    db = null;
    console.log('✅ Database connection closed');
  }
}

// Handle graceful shutdown
process.on('SIGINT', () => {
  closeDatabase();
  process.exit(0);
});

process.on('SIGTERM', () => {
  closeDatabase();
  process.exit(0);
});
