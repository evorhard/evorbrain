/**
 * Common types used across the application
 */

/**
 * A unique identifier string
 */
export type ID = string;

/**
 * ISO 8601 timestamp string
 */
export type Timestamp = string;

/**
 * Common status values for entities
 */
export type EntityStatus = 'active' | 'archived' | 'deleted';

/**
 * Task-specific status values
 */
export type TaskStatus = 'not-started' | 'in-progress' | 'completed' | 'cancelled';

/**
 * Project status values
 */
export type ProjectStatus = 'planning' | 'active' | 'on-hold' | 'completed' | 'cancelled';

/**
 * Base properties shared by all entities
 */
export interface BaseEntity {
  id: ID;
  createdAt: Timestamp;
  updatedAt: Timestamp;
  title: string;
  description?: string;
  tags?: string[];
}

/**
 * Result type for operations that can fail
 */
export type Result<T, E = Error> = 
  | { success: true; data: T }
  | { success: false; error: E };

/**
 * Nullable type helper
 */
export type Nullable<T> = T | null;

/**
 * Deep partial type helper
 */
export type DeepPartial<T> = T extends object
  ? { [P in keyof T]?: DeepPartial<T[P]> }
  : T;