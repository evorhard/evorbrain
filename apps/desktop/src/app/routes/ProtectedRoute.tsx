import { LoadingOverlay } from '@mantine/core';
import { Navigate, useLocation } from 'react-router-dom';

import { ROUTES } from './routes';

interface ProtectedRouteProps {
  children: React.ReactNode;
  requireAuth?: boolean;
}

export function ProtectedRoute({ children, requireAuth = false }: ProtectedRouteProps) {
  const location = useLocation();
  
  // TODO: Replace with actual auth state from store
  const isAuthenticated = true;
  const isLoading = false;

  if (isLoading) {
    return <LoadingOverlay visible />;
  }

  if (requireAuth && !isAuthenticated) {
    // Redirect to home page or login when implemented
    return <Navigate replace state={{ from: location }} to={ROUTES.HOME} />;
  }

  return <>{children}</>;
}