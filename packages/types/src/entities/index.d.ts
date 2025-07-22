/**
 * Entity type definitions
 */

import type { BaseEntity, ID, TaskStatus, ProjectStatus } from '../common/index.js';

/**
 * Area entity - Top level organizational unit
 */
export interface Area extends BaseEntity {
  type: 'area';
  icon?: string;
  color?: string;
  sortOrder: number;
}

/**
 * Goal entity - Belongs to an Area
 */
export interface Goal extends BaseEntity {
  type: 'goal';
  areaId: ID;
  targetDate?: string;
  progress: number; // 0-100
  status: 'active' | 'completed' | 'abandoned';
  sortOrder: number;
}

/**
 * Project entity - Belongs to a Goal
 */
export interface Project extends BaseEntity {
  type: 'project';
  goalId: ID;
  status: ProjectStatus;
  startDate?: string;
  endDate?: string;
  progress: number; // 0-100
  sortOrder: number;
}

/**
 * Task entity - Can belong to a Project or be standalone
 */
export interface Task extends BaseEntity {
  type: 'task';
  projectId?: ID;
  parentTaskId?: ID;
  status: TaskStatus;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  dueDate?: string;
  completedAt?: string;
  estimatedMinutes?: number;
  actualMinutes?: number;
  sortOrder: number;
  
  // Recurring task properties
  recurrence?: RecurrenceRule;
  recurrenceId?: ID; // ID of the recurrence series
  recurrenceDate?: string; // Date of this specific instance
}

/**
 * Recurrence rule for tasks
 */
export interface RecurrenceRule {
  frequency: 'daily' | 'weekly' | 'monthly' | 'yearly';
  interval: number;
  endDate?: string;
  endCount?: number;
  weekDays?: number[]; // 0-6, Sunday to Saturday
  monthDay?: number; // 1-31
  exceptions?: string[]; // ISO date strings
}

/**
 * Union type of all entities
 */
export type Entity = Area | Goal | Project | Task;

/**
 * Entity type string literal union
 */
export type EntityType = Entity['type'];