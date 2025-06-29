import { Hono } from 'hono';
import type { Context } from 'hono';

const app = new Hono();

// Life Areas routes
app.get('/life-areas', async (c: Context) => {
  return c.json({
    success: true,
    data: [],
    meta: { total: 0, page: 1, limit: 50 },
    timestamp: new Date().toISOString(),
  });
});

app.post('/life-areas', async (c: Context) => {
  return c.json(
    {
      success: true,
      data: { id: crypto.randomUUID(), name: 'New Life Area' },
      timestamp: new Date().toISOString(),
    },
    201
  );
});

// Goals routes
app.get('/goals', async (c: Context) => {
  return c.json({
    success: true,
    data: [],
    meta: { total: 0, page: 1, limit: 50 },
    timestamp: new Date().toISOString(),
  });
});

app.post('/goals', async (c: Context) => {
  return c.json(
    {
      success: true,
      data: { id: crypto.randomUUID(), name: 'New Goal' },
      timestamp: new Date().toISOString(),
    },
    201
  );
});

// Projects routes
app.get('/projects', async (c: Context) => {
  return c.json({
    success: true,
    data: [],
    meta: { total: 0, page: 1, limit: 50 },
    timestamp: new Date().toISOString(),
  });
});

app.post('/projects', async (c: Context) => {
  return c.json(
    {
      success: true,
      data: { id: crypto.randomUUID(), name: 'New Project' },
      timestamp: new Date().toISOString(),
    },
    201
  );
});

// Tasks routes
app.get('/tasks', async (c: Context) => {
  return c.json({
    success: true,
    data: [],
    meta: { total: 0, page: 1, limit: 50 },
    timestamp: new Date().toISOString(),
  });
});

app.post('/tasks', async (c: Context) => {
  return c.json(
    {
      success: true,
      data: { id: crypto.randomUUID(), name: 'New Task' },
      timestamp: new Date().toISOString(),
    },
    201
  );
});

export { app as apiRoutes };
