import { z } from 'zod';

// Base validation schemas
export const BaseEntitySchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(255),
  description: z.string().max(1000).optional(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  archived: z.boolean().default(false),
});

export const LifeAreaSchema = BaseEntitySchema.extend({
  color: z.string().regex(/^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/),
  sortOrder: z.number().int().min(0).default(0),
});

export const GoalSchema = BaseEntitySchema.extend({
  lifeAreaId: z.string().uuid(),
  status: z
    .enum(['active', 'completed', 'paused', 'cancelled'])
    .default('active'),
  targetDate: z.string().date().optional(),
  sortOrder: z.number().int().min(0).default(0),
});

export const ProjectSchema = BaseEntitySchema.extend({
  goalId: z.string().uuid(),
  status: z
    .enum(['planning', 'active', 'completed', 'paused', 'cancelled'])
    .default('planning'),
  startDate: z.string().date().optional(),
  targetDate: z.string().date().optional(),
  sortOrder: z.number().int().min(0).default(0),
});

export const TaskSchema = BaseEntitySchema.extend({
  projectId: z.string().uuid(),
  status: z
    .enum(['todo', 'in_progress', 'completed', 'blocked'])
    .default('todo'),
  priority: z.enum(['low', 'medium', 'high', 'urgent']).default('medium'),
  dueDate: z.string().date().optional(),
  completedAt: z.string().datetime().optional(),
  sortOrder: z.number().int().min(0).default(0),
});

export const RelationshipSchema = z.object({
  id: z.string().uuid(),
  fromType: z.enum(['life_area', 'goal', 'project', 'task']),
  fromId: z.string().uuid(),
  toType: z.enum(['life_area', 'goal', 'project', 'task']),
  toId: z.string().uuid(),
  relationshipType: z.enum(['depends_on', 'blocks', 'relates_to']),
  createdAt: z.string().datetime(),
});

export const MetadataSchema = z.object({
  entityType: z.enum(['life_area', 'goal', 'project', 'task']),
  entityId: z.string().uuid(),
  key: z.string().min(1).max(100),
  value: z.string().max(1000),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

// Create schemas (without timestamps and generated fields)
export const CreateLifeAreaSchema = LifeAreaSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export const CreateGoalSchema = GoalSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export const CreateProjectSchema = ProjectSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export const CreateTaskSchema = TaskSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
  completedAt: true,
});

// Update schemas (partial updates)
export const UpdateLifeAreaSchema = CreateLifeAreaSchema.partial();
export const UpdateGoalSchema = CreateGoalSchema.partial();
export const UpdateProjectSchema = CreateProjectSchema.partial();
export const UpdateTaskSchema = CreateTaskSchema.partial();

// API request/response schemas
export const ApiResponseSchema = <T extends z.ZodTypeAny>(dataSchema: T) =>
  z.object({
    success: z.boolean(),
    data: dataSchema.optional(),
    error: z
      .object({
        message: z.string(),
        code: z.string(),
        details: z.any().optional(),
      })
      .optional(),
    timestamp: z.string().datetime(),
  });

export const PaginationSchema = z.object({
  page: z.number().int().min(1).default(1),
  limit: z.number().int().min(1).max(100).default(20),
  total: z.number().int().min(0),
  hasNext: z.boolean(),
  hasPrev: z.boolean(),
});

export const PaginatedResponseSchema = <T extends z.ZodTypeAny>(
  itemSchema: T
) =>
  ApiResponseSchema(
    z.object({
      items: z.array(itemSchema),
      pagination: PaginationSchema,
    })
  );

// WebSocket message schemas
export const WebSocketMessageSchema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('item_created'),
    payload: z
      .object({
        entityType: z.string(),
      })
      .and(z.union([LifeAreaSchema, GoalSchema, ProjectSchema, TaskSchema])),
  }),
  z.object({
    type: z.literal('item_updated'),
    payload: z
      .object({
        entityType: z.string(),
      })
      .and(z.union([LifeAreaSchema, GoalSchema, ProjectSchema, TaskSchema])),
  }),
  z.object({
    type: z.literal('item_deleted'),
    payload: z.object({
      id: z.string().uuid(),
      entityType: z.string(),
    }),
  }),
  z.object({
    type: z.literal('status_changed'),
    payload: z.object({
      id: z.string().uuid(),
      entityType: z.string(),
      status: z.string(),
    }),
  }),
  z.object({
    type: z.literal('reorder'),
    payload: z.object({
      parentId: z.string().uuid(),
      entityType: z.string(),
      newOrder: z.array(z.string().uuid()),
    }),
  }),
  z.object({
    type: z.literal('connection_status'),
    payload: z.object({
      connected: z.boolean(),
      clientCount: z.number().int().min(0),
    }),
  }),
]);
