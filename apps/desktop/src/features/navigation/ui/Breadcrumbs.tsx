import { Breadcrumbs as MantineBreadcrumbs, Anchor, Text } from '@mantine/core';
import { IconHome } from '@tabler/icons-react';
import { useLocation, Link } from 'react-router-dom';

import { ROUTES } from '../../../app/routes/routes';

interface BreadcrumbItem {
  label: string;
  path?: string;
}

export function Breadcrumbs() {
  const location = useLocation();

  const getBreadcrumbs = (): BreadcrumbItem[] => {
    const items: BreadcrumbItem[] = [{ label: 'Home', path: ROUTES.HOME }];

    // Parse current path to build breadcrumbs
    const path = location.pathname;

    if (path === ROUTES.DAILY_VIEW) {
      items.push({ label: 'Daily View' });
    } else if (path === ROUTES.CALENDAR_VIEW) {
      items.push({ label: 'Calendar' });
    } else if (path === ROUTES.HIERARCHY_VIEW) {
      items.push({ label: 'Hierarchy' });
    } else if (path === ROUTES.SETTINGS) {
      items.push({ label: 'Settings' });
    } else if (path.startsWith('/area/')) {
      items.push({ label: 'Areas', path: ROUTES.HIERARCHY_VIEW });
      items.push({ label: 'Area Detail' });
    } else if (path.startsWith('/goal/')) {
      items.push({ label: 'Goals', path: ROUTES.HIERARCHY_VIEW });
      items.push({ label: 'Goal Detail' });
    } else if (path.startsWith('/project/')) {
      items.push({ label: 'Projects', path: ROUTES.HIERARCHY_VIEW });
      items.push({ label: 'Project Detail' });
    } else if (path.startsWith('/task/')) {
      items.push({ label: 'Tasks', path: ROUTES.DAILY_VIEW });
      items.push({ label: 'Task Detail' });
    } else if (path === ROUTES.SEARCH) {
      items.push({ label: 'Search Results' });
    }

    return items;
  };

  const items = getBreadcrumbs();

  return (
    <MantineBreadcrumbs separator="→" separatorMargin="md">
      {items.map((item, index) => {
        const isLast = index === items.length - 1;

        if (isLast || !item.path) {
          return (
            <Text key={index} c="dimmed" size="sm">
              {item.label}
            </Text>
          );
        }

        return (
          <Anchor
            key={index}
            c="dimmed"
            className="hover:underline"
            component={Link}
            size="sm"
            to={item.path}
          >
            {index === 0 && <IconHome className="inline mr-1" size={14} />}
            {item.label}
          </Anchor>
        );
      })}
    </MantineBreadcrumbs>
  );
}
