/**
 * API and IPC type definitions
 */

import type { Result } from '../common/index.js';
import type { Entity, EntityType } from '../entities/index.js';

/**
 * IPC command names
 */
export type IPCCommand =
  | 'get_entities'
  | 'create_entity'
  | 'update_entity'
  | 'delete_entity'
  | 'search'
  | 'get_file_content'
  | 'save_file_content'
  | 'watch_directory'
  | 'unwatch_directory';

/**
 * Search query parameters
 */
export interface SearchQuery {
  query: string;
  entityTypes?: EntityType[];
  tags?: string[];
  dateRange?: {
    start: string;
    end: string;
  };
  limit?: number;
  offset?: number;
}

/**
 * Search result item
 */
export interface SearchResult {
  entity: Entity;
  score: number;
  highlights: string[];
}

/**
 * File operation parameters
 */
export interface FileOperation {
  path: string;
  content?: string;
  backup?: boolean;
}

/**
 * Directory watch event
 */
export interface WatchEvent {
  type: 'created' | 'modified' | 'deleted' | 'renamed';
  path: string;
  oldPath?: string; // For rename events
}

/**
 * API response types
 */
export type APIResponse<T> = Result<T, APIError>;

/**
 * API error structure
 */
export interface APIError {
  code: string;
  message: string;
  details?: unknown;
}
