import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { Dashboard } from './pages/Dashboard';
import { LifeAreas } from './pages/LifeAreas';
import { Goals } from './pages/Goals';
import { Projects } from './pages/Projects';
import { Tasks } from './pages/Tasks';
import { Settings } from './pages/Settings';

function App(): React.ReactElement {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/life-areas" element={<LifeAreas />} />
        <Route path="/goals" element={<Goals />} />
        <Route path="/projects" element={<Projects />} />
        <Route path="/tasks" element={<Tasks />} />
        <Route path="/settings" element={<Settings />} />
        {/* Hierarchical routes */}
        <Route path="/life-areas/:lifeAreaId/goals" element={<Goals />} />
        <Route path="/goals/:goalId/projects" element={<Projects />} />
        <Route path="/projects/:projectId/tasks" element={<Tasks />} />
      </Routes>
    </Layout>
  );
}

export default App;
