/**
 * Utility type definitions
 */

/**
 * Extract the type of array elements
 */
export type ArrayElement<T> = T extends readonly (infer U)[] ? U : never;

/**
 * Make specific keys optional
 */
export type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

/**
 * Make specific keys required
 */
export type RequiredBy<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

/**
 * Exclude null and undefined from T
 */
export type NonNullable<T> = T extends null | undefined ? never : T;

/**
 * Get the keys of T that are optional
 */
export type OptionalKeys<T> = {
  [K in keyof T]-?: object extends Pick<T, K> ? K : never;
}[keyof T];

/**
 * Get the keys of T that are required
 */
export type RequiredKeys<T> = Exclude<keyof T, OptionalKeys<T>>;

/**
 * Create a type with at least one property from T
 */
export type AtLeastOne<T> = {
  [K in keyof T]: Pick<T, K> & Partial<Omit<T, K>>;
}[keyof T];

/**
 * Create a union type from object values
 */
export type ValueOf<T> = T[keyof T];

/**
 * Async function type
 */
export type AsyncFunction<T extends unknown[] = unknown[], R = void> = (...args: T) => Promise<R>;

/**
 * Type guard function
 */
export type TypeGuard<T> = (value: unknown) => value is T;

/**
 * Branded type for type-safe IDs
 */
export type Brand<K, T> = K & { __brand: T };

/**
 * Common branded types
 */
export type AreaId = Brand<string, 'AreaId'>;
export type GoalId = Brand<string, 'GoalId'>;
export type ProjectId = Brand<string, 'ProjectId'>;
export type TaskId = Brand<string, 'TaskId'>;
