import { useCallback } from 'react';

import { useNavigate, useLocation } from 'react-router-dom';

import { ROUTES } from '../../../app/routes/routes';
import { buildEntityRoute, buildSearchRoute } from '../../../app/routes/utils';

export function useNavigation() {
  const navigate = useNavigate();
  const location = useLocation();

  const navigateToEntity = useCallback(
    (type: 'area' | 'goal' | 'project' | 'task', id: string) => {
      const route = buildEntityRoute(type, id);
      void navigate(route);
    },
    [navigate]
  );

  const navigateToSearch = useCallback(
    (query: string) => {
      const route = buildSearchRoute(query);
      void navigate(route);
    },
    [navigate]
  );

  const navigateToDaily = useCallback(() => {
    void navigate(ROUTES.DAILY_VIEW);
  }, [navigate]);

  const navigateToCalendar = useCallback(() => {
    void navigate(ROUTES.CALENDAR_VIEW);
  }, [navigate]);

  const navigateToHierarchy = useCallback(() => {
    void navigate(ROUTES.HIERARCHY_VIEW);
  }, [navigate]);

  const navigateToSettings = useCallback(() => {
    void navigate(ROUTES.SETTINGS);
  }, [navigate]);

  const goBack = useCallback(() => {
    void navigate(-1);
  }, [navigate]);

  return {
    // Current location info
    currentPath: location.pathname,
    isDaily: location.pathname === ROUTES.DAILY_VIEW,
    isCalendar: location.pathname === ROUTES.CALENDAR_VIEW,
    isHierarchy: location.pathname === ROUTES.HIERARCHY_VIEW,
    isSettings: location.pathname === ROUTES.SETTINGS,
    
    // Navigation functions
    navigate,
    navigateToEntity,
    navigateToSearch,
    navigateToDaily,
    navigateToCalendar,
    navigateToHierarchy,
    navigateToSettings,
    goBack,
  };
}