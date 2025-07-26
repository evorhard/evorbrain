export const ROUTES = {
  HOME: '/',
  DAILY_VIEW: '/daily',
  CALENDAR_VIEW: '/calendar',
  HIERARCHY_VIEW: '/hierarchy',
  SETTINGS: '/settings',
  // Entity list routes
  AREAS_LIST: '/areas',
  GOALS_LIST: '/goals',
  PROJECTS_LIST: '/projects',
  TASKS_LIST: '/tasks',
  // Entity detail routes
  AREA: '/area/:id',
  GOAL: '/goal/:id',
  PROJECT: '/project/:id',
  TASK: '/task/:id',
  // Search
  SEARCH: '/search',
} as const;

export type RouteKey = keyof typeof ROUTES;
export type RouteValue = (typeof ROUTES)[RouteKey];
