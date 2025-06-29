import React from 'react';
import { Plus, Target } from 'lucide-react';

export function LifeAreas(): React.ReactElement {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Life Areas</h1>
          <p className="text-gray-600">
            Organize your life into meaningful areas
          </p>
        </div>
        <button className="btn-primary flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>New Life Area</span>
        </button>
      </div>

      {/* Empty State */}
      <div className="card p-12 text-center">
        <Target className="h-16 w-16 text-gray-400 mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          No Life Areas Yet
        </h2>
        <p className="text-gray-600 mb-6">
          Create your first life area to start organizing your goals and
          projects.
        </p>
        <button className="btn-primary">Create First Life Area</button>
      </div>
    </div>
  );
}
