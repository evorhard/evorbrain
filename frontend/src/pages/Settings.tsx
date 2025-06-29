import React from 'react';
import { Settings as SettingsIcon, Database, Server, Info } from 'lucide-react';

export function Settings(): React.ReactElement {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
        <p className="text-gray-600">Configure your EvorBrain instance</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* System Information */}
        <div className="card p-6">
          <div className="flex items-center space-x-3 mb-4">
            <Info className="h-5 w-5 text-primary-600" />
            <h2 className="text-lg font-semibold text-gray-900">
              System Information
            </h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Version</span>
              <span className="font-medium">0.1.0</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Environment</span>
              <span className="font-medium">Development</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Runtime</span>
              <span className="font-medium">Bun</span>
            </div>
          </div>
        </div>

        {/* Database Settings */}
        <div className="card p-6">
          <div className="flex items-center space-x-3 mb-4">
            <Database className="h-5 w-5 text-primary-600" />
            <h2 className="text-lg font-semibold text-gray-900">Database</h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Type</span>
              <span className="font-medium">SQLite</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Status</span>
              <span className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-green-100 text-green-800">
                Connected
              </span>
            </div>
          </div>
        </div>

        {/* Server Settings */}
        <div className="card p-6">
          <div className="flex items-center space-x-3 mb-4">
            <Server className="h-5 w-5 text-primary-600" />
            <h2 className="text-lg font-semibold text-gray-900">Server</h2>
          </div>
          <div className="space-y-3">
            <div className="flex justify-between">
              <span className="text-gray-600">Backend Port</span>
              <span className="font-medium">3000</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">Frontend Port</span>
              <span className="font-medium">5173</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-600">WebSocket</span>
              <span className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-green-100 text-green-800">
                Active
              </span>
            </div>
          </div>
        </div>

        {/* Application Settings */}
        <div className="card p-6">
          <div className="flex items-center space-x-3 mb-4">
            <SettingsIcon className="h-5 w-5 text-primary-600" />
            <h2 className="text-lg font-semibold text-gray-900">Application</h2>
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Theme
              </label>
              <select className="input-field">
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="system">System</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Auto-save Interval (seconds)
              </label>
              <input
                type="number"
                min="5"
                max="300"
                defaultValue="30"
                className="input-field"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
