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

// Error boundary component
function ErrorPage() {
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
          {/* Entity routes - TODO: implement these pages */}
          <Route element={<div>Area Detail Page - TODO</div>} path={ROUTES.AREA} />
          <Route element={<div>Goal Detail Page - TODO</div>} path={ROUTES.GOAL} />
          <Route element={<div>Project Detail Page - TODO</div>} path={ROUTES.PROJECT} />
          <Route element={<div>Task Detail Page - TODO</div>} path={ROUTES.TASK} />
          <Route element={<div>Search Page - TODO</div>} path={ROUTES.SEARCH} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
