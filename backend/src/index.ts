import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { logger } from 'hono/logger';
import { secureHeaders } from 'hono/secure-headers';
import { initializeDatabase } from './database/connection';
import { apiRoutes } from './routes/index';
import { websocketHandler } from './websocket/handler';
import { errorHandler } from './middleware/error-handler';

const app = new Hono();

// Middleware
app.use('*', logger());
app.use('*', secureHeaders());
app.use(
  '*',
  cors({
    origin:
      process.env.NODE_ENV === 'production' ? [] : ['http://localhost:5173'],
    allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowHeaders: ['Content-Type', 'Authorization'],
    credentials: true,
  })
);

// Routes
app.route('/api', apiRoutes);

// Health check endpoint
app.get('/health', c => {
  return c.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: '0.1.0',
  });
});

// Error handling middleware
app.onError(errorHandler);

// 404 handler
app.notFound(c => {
  return c.json(
    {
      success: false,
      error: {
        message: 'Route not found',
        code: 'NOT_FOUND',
      },
      timestamp: new Date().toISOString(),
    },
    404
  );
});

// Initialize database
await initializeDatabase();

const PORT = Number(process.env.PORT) || 3000;

console.log(`🚀 EvorBrain backend server ready on port ${PORT}`);
console.log(`📊 Health check available at http://localhost:${PORT}/health`);
console.log(`🔌 WebSocket endpoint: ws://localhost:${PORT}/ws`);

// Export the app and configuration separately
export { app, websocketHandler };
export const serverConfig = {
  port: PORT,
  fetch: app.fetch,
  websocket: websocketHandler,
};

export default serverConfig;
