import React from 'react';
import { Plus, FolderOpen } from 'lucide-react';

export function Projects(): React.ReactElement {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Projects</h1>
          <p className="text-gray-600">Manage your projects within goals</p>
        </div>
        <button className="btn-primary flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>New Project</span>
        </button>
      </div>

      {/* Empty State */}
      <div className="card p-12 text-center">
        <FolderOpen className="h-16 w-16 text-gray-400 mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          No Projects Yet
        </h2>
        <p className="text-gray-600 mb-6">
          Create your first project to start organizing your tasks.
        </p>
        <button className="btn-primary">Create First Project</button>
      </div>
    </div>
  );
}
