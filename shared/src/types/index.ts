// Core entity types
export interface BaseEntity {
  id: string;
  name: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
  archived: boolean;
}

export interface LifeArea extends BaseEntity {
  color: string;
  sortOrder: number;
}

export interface Goal extends BaseEntity {
  lifeAreaId: string;
  status: 'active' | 'completed' | 'paused' | 'cancelled';
  targetDate?: string;
  sortOrder: number;
}

export interface Project extends BaseEntity {
  goalId: string;
  status: 'planning' | 'active' | 'completed' | 'paused' | 'cancelled';
  startDate?: string;
  targetDate?: string;
  sortOrder: number;
}

export interface Task extends BaseEntity {
  projectId: string;
  status: 'todo' | 'in_progress' | 'completed' | 'blocked';
  priority: 'low' | 'medium' | 'high' | 'urgent';
  dueDate?: string;
  completedAt?: string;
  sortOrder: number;
}

export interface Relationship {
  id: string;
  fromType: 'life_area' | 'goal' | 'project' | 'task';
  fromId: string;
  toType: 'life_area' | 'goal' | 'project' | 'task';
  toId: string;
  relationshipType: 'depends_on' | 'blocks' | 'relates_to';
  createdAt: string;
}

export interface Metadata {
  entityType: 'life_area' | 'goal' | 'project' | 'task';
  entityId: string;
  key: string;
  value: string;
  createdAt: string;
  updatedAt: string;
}

// API response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    message: string;
    code: string;
    details?: unknown;
  };
  timestamp: string;
}

export type PaginatedResponse<T> = ApiResponse<{
  items: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    hasNext: boolean;
    hasPrev: boolean;
  };
}>;

// WebSocket message types
export type WebSocketMessage =
  | { type: 'item_created'; payload: HierarchyItem & { entityType: string } }
  | { type: 'item_updated'; payload: HierarchyItem & { entityType: string } }
  | { type: 'item_deleted'; payload: { id: string; entityType: string } }
  | {
      type: 'status_changed';
      payload: { id: string; entityType: string; status: string };
    }
  | {
      type: 'reorder';
      payload: { parentId: string; entityType: string; newOrder: string[] };
    }
  | {
      type: 'connection_status';
      payload: { connected: boolean; clientCount: number };
    };

export type HierarchyItem = LifeArea | Goal | Project | Task;
export type EntityType = 'life_area' | 'goal' | 'project' | 'task';
export type Status = Goal['status'] | Project['status'] | Task['status'];
export type Priority = Task['priority'];
