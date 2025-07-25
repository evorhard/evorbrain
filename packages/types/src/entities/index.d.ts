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
 * Area entity with methods for business logic
 */
export interface AreaWithMethods extends Area {
  // Query methods
  getGoals(): Promise<Goal[]>;
  getActiveGoals(): Promise<Goal[]>;
  getCompletedGoals(): Promise<Goal[]>;
  getAllTasks(): Promise<Task[]>;
  getTasksByStatus(status: TaskStatus): Promise<Task[]>;
  
  // Calculation methods
  calculateProgress(): number;
  calculateActiveProjectsCount(): number;
  calculateCompletedTasksCount(): number;
  
  // Validation methods
  canBeDeleted(): boolean;
  canBeArchived(): boolean;
  validate(): string[];
  
  // Relationship methods
  addGoal(goal: Partial<Goal>): Promise<Goal>;
  removeGoal(goalId: ID): Promise<void>;
  reorderGoals(goalIds: ID[]): Promise<void>;
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
 * Goal entity with methods for business logic
 */
export interface GoalWithMethods extends Goal {
  // Query methods
  getArea(): Promise<Area>;
  getProjects(): Promise<Project[]>;
  getActiveProjects(): Promise<Project[]>;
  getCompletedProjects(): Promise<Project[]>;
  getAllTasks(): Promise<Task[]>;
  
  // Calculation methods
  calculateProgress(): number;
  calculateTimeRemaining(): number | null;
  isOverdue(): boolean;
  getDaysUntilTarget(): number | null;
  
  // Status methods
  markAsCompleted(): Promise<void>;
  markAsAbandoned(): Promise<void>;
  reactivate(): Promise<void>;
  
  // Validation methods
  canBeCompleted(): boolean;
  canBeDeleted(): boolean;
  validate(): string[];
  
  // Relationship methods
  addProject(project: Partial<Project>): Promise<Project>;
  removeProject(projectId: ID): Promise<void>;
  updateProgress(progress: number): Promise<void>;
  reorderProjects(projectIds: ID[]): Promise<void>;
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
 * Project entity with methods for business logic
 */
export interface ProjectWithMethods extends Project {
  // Query methods
  getGoal(): Promise<Goal>;
  getTasks(): Promise<Task[]>;
  getTasksByStatus(status: TaskStatus): Promise<Task[]>;
  getOverdueTasks(): Promise<Task[]>;
  getUpcomingTasks(days: number): Promise<Task[]>;
  
  // Calculation methods
  calculateProgress(): number;
  calculateCompletionPercentage(): number;
  getEstimatedCompletionDate(): Date | null;
  isOverdue(): boolean;
  getDaysRemaining(): number | null;
  
  // Status methods
  start(): Promise<void>;
  complete(): Promise<void>;
  cancel(): Promise<void>;
  putOnHold(): Promise<void>;
  resume(): Promise<void>;
  
  // Validation methods
  canBeStarted(): boolean;
  canBeCompleted(): boolean;
  canBeDeleted(): boolean;
  validate(): string[];
  
  // Relationship methods
  addTask(task: Partial<Task>): Promise<Task>;
  removeTask(taskId: ID): Promise<void>;
  updateProgress(progress: number): Promise<void>;
  reorderTasks(taskIds: ID[]): Promise<void>;
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
 * Task entity with methods for business logic
 */
export interface TaskWithMethods extends Task {
  // Query methods
  getProject(): Promise<Project | null>;
  getParentTask(): Promise<Task | null>;
  getSubtasks(): Promise<Task[]>;
  getRecurrenceInstances(): Promise<Task[]>;
  
  // Calculation methods
  isOverdue(): boolean;
  getDaysUntilDue(): number | null;
  getTimeSpent(): number;
  getTimeRemaining(): number | null;
  hasSubtasks(): boolean;
  isRecurring(): boolean;
  isRecurrenceInstance(): boolean;
  
  // Status methods
  start(): Promise<void>;
  complete(): Promise<void>;
  cancel(): Promise<void>;
  reopen(): Promise<void>;
  
  // Priority methods
  setPriority(priority: Task['priority']): Promise<void>;
  increasePriority(): Promise<void>;
  decreasePriority(): Promise<void>;
  
  // Time tracking methods
  startTimer(): Promise<void>;
  stopTimer(): Promise<void>;
  logTime(minutes: number): Promise<void>;
  
  // Recurrence methods
  createNextInstance(): Promise<Task | null>;
  updateRecurrenceRule(rule: RecurrenceRule): Promise<void>;
  deleteRecurrenceSeries(): Promise<void>;
  detachFromRecurrence(): Promise<void>;
  
  // Validation methods
  canBeCompleted(): boolean;
  canBeDeleted(): boolean;
  canHaveSubtasks(): boolean;
  validate(): string[];
  
  // Relationship methods
  addSubtask(subtask: Partial<Task>): Promise<Task>;
  removeSubtask(subtaskId: ID): Promise<void>;
  moveToProject(projectId: ID | null): Promise<void>;
  convertToProject(): Promise<Project>;
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
 * Union type of all entities with methods
 */
export type EntityWithMethods = AreaWithMethods | GoalWithMethods | ProjectWithMethods | TaskWithMethods;

/**
 * Entity type string literal union
 */
export type EntityType = Entity['type'];

/**
 * Shared entity base interface with common methods
 */
export interface BaseEntityMethods {
  // Persistence methods
  save(): Promise<void>;
  delete(): Promise<void>;
  reload(): Promise<void>;
  
  // Metadata methods
  touch(): Promise<void>;
  addTag(tag: string): Promise<void>;
  removeTag(tag: string): Promise<void>;
  hasTag(tag: string): boolean;
  
  // Export methods
  toMarkdown(): string;
  toJSON(): Record<string, unknown>;
  clone(): Promise<Entity>;
  
  // History methods
  getHistory(): Promise<EntityHistory[]>;
  revertTo(version: string): Promise<void>;
}

/**
 * Entity history record
 */
export interface EntityHistory {
  version: string;
  timestamp: string;
  changes: Record<string, unknown>;
  author?: string;
}