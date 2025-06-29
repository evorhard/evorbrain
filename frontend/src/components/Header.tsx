import React from 'react';
import { Brain, Settings } from 'lucide-react';
import { Link } from 'react-router-dom';

export function Header(): React.ReactElement {
  return (
    <header className="bg-white border-b border-gray-200 px-6 py-4">
      <div className="flex items-center justify-between max-w-7xl mx-auto">
        <div className="flex items-center space-x-3">
          <Brain className="h-8 w-8 text-primary-600" />
          <div>
            <h1 className="text-xl font-bold text-gray-900">EvorBrain</h1>
            <p className="text-sm text-gray-500">Hierarchical Second Brain</p>
          </div>
        </div>

        <div className="flex items-center space-x-4">
          <div className="text-sm text-gray-600">
            <span className="font-medium">Local Instance</span>
            <span className="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-800">
              Online
            </span>
          </div>

          <Link
            to="/settings"
            className="p-2 text-gray-400 hover:text-gray-600 transition-colors duration-200"
            title="Settings"
          >
            <Settings className="h-5 w-5" />
          </Link>
        </div>
      </div>
    </header>
  );
}
