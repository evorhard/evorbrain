import type { Context } from 'hono';
import type { StatusCode } from 'hono/utils/http-status';

export interface AppError extends Error {
  statusCode?: number;
  code?: string;
  details?: Record<string, unknown>;
}

export function errorHandler(error: Error | AppError, c: Context): Response {
  console.error('Error occurred:', {
    message: error.message,
    stack: error.stack,
    path: c.req.path,
    method: c.req.method,
  });

  // Handle known application errors
  if ('statusCode' in error && error.statusCode) {
    return c.json(
      {
        success: false,
        error: {
          message: error.message,
          code: error.code || 'APP_ERROR',
          details: error.details,
        },
        timestamp: new Date().toISOString(),
      },
      error.statusCode as StatusCode
    );
  }

  // Handle validation errors from Zod
  if (error.name === 'ZodError') {
    return c.json(
      {
        success: false,
        error: {
          message: 'Validation failed',
          code: 'VALIDATION_ERROR',
          details: error,
        },
        timestamp: new Date().toISOString(),
      },
      400
    );
  }

  // Handle database errors
  if (error.message.includes('SQLITE_CONSTRAINT')) {
    return c.json(
      {
        success: false,
        error: {
          message: 'Database constraint violation',
          code: 'CONSTRAINT_ERROR',
          details: { message: error.message },
        },
        timestamp: new Date().toISOString(),
      },
      409
    );
  }

  // Handle generic errors
  return c.json(
    {
      success: false,
      error: {
        message:
          process.env.NODE_ENV === 'production'
            ? 'Internal server error'
            : error.message,
        code: 'INTERNAL_ERROR',
      },
      timestamp: new Date().toISOString(),
    },
    500
  );
}

export function createAppError(
  message: string,
  statusCode: number = 500,
  code?: string,
  details?: Record<string, unknown>
): AppError {
  const error = new Error(message) as AppError;
  error.statusCode = statusCode;
  error.code = code;
  error.details = details;
  return error;
}
