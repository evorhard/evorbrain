import React from 'react';
import { Plus, CheckSquare } from 'lucide-react';

export function Tasks(): React.ReactElement {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Tasks</h1>
          <p className="text-gray-600">Manage your tasks within projects</p>
        </div>
        <button className="btn-primary flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>New Task</span>
        </button>
      </div>

      {/* Empty State */}
      <div className="card p-12 text-center">
        <CheckSquare className="h-16 w-16 text-gray-400 mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          No Tasks Yet
        </h2>
        <p className="text-gray-600 mb-6">
          Create your first task to start getting things done.
        </p>
        <button className="btn-primary">Create First Task</button>
      </div>
    </div>
  );
}
