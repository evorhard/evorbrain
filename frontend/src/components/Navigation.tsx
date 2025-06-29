import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
  Home,
  Target,
  Bookmark,
  FolderOpen,
  CheckSquare,
  BarChart3,
} from 'lucide-react';
import { clsx } from 'clsx';

const navigationItems = [
  {
    name: 'Dashboard',
    href: '/',
    icon: Home,
  },
  {
    name: 'Life Areas',
    href: '/life-areas',
    icon: Target,
  },
  {
    name: 'Goals',
    href: '/goals',
    icon: Bookmark,
  },
  {
    name: 'Projects',
    href: '/projects',
    icon: FolderOpen,
  },
  {
    name: 'Tasks',
    href: '/tasks',
    icon: CheckSquare,
  },
  {
    name: 'Analytics',
    href: '/analytics',
    icon: BarChart3,
  },
];

export function Navigation(): React.ReactElement {
  const location = useLocation();

  return (
    <nav className="w-64 bg-white border-r border-gray-200 min-h-screen">
      <div className="p-4">
        <h2 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-4">
          Navigation
        </h2>
        <ul className="space-y-1">
          {navigationItems.map(item => {
            const isActive = location.pathname === item.href;
            const Icon = item.icon;

            return (
              <li key={item.name}>
                <Link
                  to={item.href}
                  className={clsx(
                    'flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors duration-200',
                    isActive
                      ? 'bg-primary-100 text-primary-700'
                      : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                  )}
                >
                  <Icon className="h-5 w-5" />
                  <span>{item.name}</span>
                </Link>
              </li>
            );
          })}
        </ul>
      </div>
    </nav>
  );
}
