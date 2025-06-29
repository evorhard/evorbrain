import React from 'react';
import { Navigation } from './Navigation';
import { Header } from './Header';

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps): React.ReactElement {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <div className="flex">
        <Navigation />
        <main className="flex-1 p-6">
          <div className="max-w-7xl mx-auto">{children}</div>
        </main>
      </div>
    </div>
  );
}
