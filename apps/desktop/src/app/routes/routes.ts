export const ROUTES = {
  HOME: '/',
  DAILY_VIEW: '/daily',
  CALENDAR_VIEW: '/calendar',
  HIERARCHY_VIEW: '/hierarchy',
  SETTINGS: '/settings',
  // Entity routes
  AREA: '/area/:id',
  GOAL: '/goal/:id',
  PROJECT: '/project/:id',
  TASK: '/task/:id',
  // Search
  SEARCH: '/search',
} as const;

export type RouteKey = keyof typeof ROUTES;
export type RouteValue = (typeof ROUTES)[RouteKey];