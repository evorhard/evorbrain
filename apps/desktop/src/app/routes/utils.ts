import { ROUTES } from './routes';

// Helper to build entity routes with IDs
export function buildEntityRoute(
  type: 'area' | 'goal' | 'project' | 'task',
  id: string
): string {
  const routeMap = {
    area: ROUTES.AREA,
    goal: ROUTES.GOAL,
    project: ROUTES.PROJECT,
    task: ROUTES.TASK,
  };

  return routeMap[type].replace(':id', id);
}

// Helper to build search route with query
export function buildSearchRoute(query: string): string {
  return `${ROUTES.SEARCH}?q=${encodeURIComponent(query)}`;
}

// Check if a path is active (for navigation highlighting)
export function isRouteActive(currentPath: string, targetPath: string): boolean {
  // Exact match
  if (currentPath === targetPath) {return true;}
  
  // Check if current path starts with target (for nested routes)
  if (targetPath !== '/' && currentPath.startsWith(targetPath)) {return true;}
  
  return false;
}