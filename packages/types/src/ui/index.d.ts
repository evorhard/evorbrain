/**
 * UI-specific type definitions
 */

import type { EntityType } from '../entities/index.js';

/**
 * Theme color variants
 */
export type ThemeColor = 
  | 'primary'
  | 'secondary'
  | 'success'
  | 'warning'
  | 'error'
  | 'info';

/**
 * Component size variants
 */
export type ComponentSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

/**
 * View modes for different screens
 */
export type ViewMode = 'grid' | 'list' | 'kanban' | 'calendar' | 'timeline';

/**
 * Sidebar navigation item
 */
export interface NavItem {
  id: string;
  label: string;
  icon?: string;
  href?: string;
  onClick?: () => void;
  children?: NavItem[];
  badge?: number | string;
  entityType?: EntityType;
}

/**
 * Toast notification options
 */
export interface ToastOptions {
  title: string;
  message?: string;
  type?: 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}

/**
 * Modal configuration
 */
export interface ModalConfig {
  title: string;
  size?: ComponentSize;
  closeOnClickOutside?: boolean;
  closeOnEsc?: boolean;
  showCloseButton?: boolean;
}

/**
 * Form field validation
 */
export interface FieldValidation {
  required?: boolean;
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  custom?: (value: unknown) => string | undefined;
}

/**
 * Pagination state
 */
export interface PaginationState {
  page: number;
  pageSize: number;
  total: number;
}

/**
 * Sort configuration
 */
export interface SortConfig {
  field: string;
  direction: 'asc' | 'desc';
}