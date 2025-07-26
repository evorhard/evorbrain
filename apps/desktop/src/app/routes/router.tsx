import { lazy, Suspense } from 'react';

import { LoadingOverlay } from '@mantine/core';
import { BrowserRouter, Routes, Route, Navigate, Outlet } from 'react-router-dom';

import { ROUTES } from './routes';
import { AppLayout } from '../layout/AppLayout';

// Lazy load pages
const DailyViewPage = lazy(() => import('../../pages/daily-view/DailyViewPage'));
const CalendarViewPage = lazy(() => import('../../pages/calendar-view/CalendarViewPage'));
const HierarchyViewPage = lazy(() => import('../../pages/hierarchy-view/HierarchyViewPage'));
const SettingsPage = lazy(() => import('../../pages/settings/SettingsPage'));
const AreaDetailPage = lazy(() => import('../../pages/area-detail/AreaDetailPage'));
const GoalDetailPage = lazy(() => import('../../pages/goal-detail/GoalDetailPage'));
const ProjectDetailPage = lazy(() => import('../../pages/project-detail/ProjectDetailPage'));
const TaskDetailPage = lazy(() => import('../../pages/task-detail/TaskDetailPage'));
const SearchPage = lazy(() => import('../../pages/search/SearchPage'));
const AreasListPage = lazy(() => import('../../pages/areas-list/AreasListPage'));
const GoalsListPage = lazy(() => import('../../pages/goals-list/GoalsListPage'));
const ProjectsListPage = lazy(() => import('../../pages/projects-list/ProjectsListPage'));
const TasksListPage = lazy(() => import('../../pages/tasks-list/TasksListPage'));

// Loading component
function PageLoader() {
  return <LoadingOverlay visible loaderProps={{ type: 'dots' }} />;
}

// Layout wrapper
function RootLayout() {
  return (
    <AppLayout>
      <Suspense fallback={<PageLoader />}>
        <Outlet />
      </Suspense>
    </AppLayout>
  );
}

// Error boundary component - kept for future use
function _ErrorPage() {
  return (
    <div className="flex h-screen items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-gray-900 dark:text-gray-100">Oops!</h1>
        <p className="mt-4 text-lg text-gray-600 dark:text-gray-400">
          Something went wrong. Please try refreshing the page.
        </p>
      </div>
    </div>
  );
}

// Router Provider component
export function AppRouter() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<RootLayout />} path="/">
          <Route index element={<Navigate replace to={ROUTES.DAILY_VIEW} />} />
          <Route element={<DailyViewPage />} path={ROUTES.DAILY_VIEW} />
          <Route element={<CalendarViewPage />} path={ROUTES.CALENDAR_VIEW} />
          <Route element={<HierarchyViewPage />} path={ROUTES.HIERARCHY_VIEW} />
          <Route element={<SettingsPage />} path={ROUTES.SETTINGS} />
          {/* Entity list routes */}
          <Route element={<AreasListPage />} path={ROUTES.AREAS_LIST} />
          <Route element={<GoalsListPage />} path={ROUTES.GOALS_LIST} />
          <Route element={<ProjectsListPage />} path={ROUTES.PROJECTS_LIST} />
          <Route element={<TasksListPage />} path={ROUTES.TASKS_LIST} />
          {/* Entity detail routes */}
          <Route element={<AreaDetailPage />} path={ROUTES.AREA} />
          <Route element={<GoalDetailPage />} path={ROUTES.GOAL} />
          <Route element={<ProjectDetailPage />} path={ROUTES.PROJECT} />
          <Route element={<TaskDetailPage />} path={ROUTES.TASK} />
          <Route element={<SearchPage />} path={ROUTES.SEARCH} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
