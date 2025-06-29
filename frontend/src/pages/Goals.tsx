import React from 'react';
import { Plus, Brain } from 'lucide-react';

export function Goals(): React.ReactElement {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Goals</h1>
          <p className="text-gray-600">Track your goals within life areas</p>
        </div>
        <button className="btn-primary flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>New Goal</span>
        </button>
      </div>

      {/* Empty State */}
      <div className="card p-12 text-center">
        <Brain className="h-16 w-16 text-gray-400 mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-gray-900 mb-2">
          No Goals Yet
        </h2>
        <p className="text-gray-600 mb-6">
          Create your first goal to start tracking your progress.
        </p>
        <button className="btn-primary">Create First Goal</button>
      </div>
    </div>
  );
}
