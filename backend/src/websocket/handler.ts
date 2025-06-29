import type { ServerWebSocket } from 'bun';

// Define WebSocket message types locally for now
interface WebSocketMessage {
  type: string;
  timestamp: string;
  data: unknown;
}

export interface WebSocketData {
  id: string;
  connectedAt: Date;
}

const connections = new Map<string, ServerWebSocket<WebSocketData>>();

export const websocketHandler = {
  message(ws: ServerWebSocket<WebSocketData>, message: string | Buffer) {
    try {
      const data = JSON.parse(message.toString()) as WebSocketMessage;

      console.log('WebSocket message received:', {
        type: data.type,
        clientId: ws.data.id,
        timestamp: data.timestamp,
      });

      // Handle different message types
      switch (data.type) {
        case 'ping':
          ws.send(
            JSON.stringify({
              type: 'pong',
              timestamp: new Date().toISOString(),
              data: null,
            } satisfies WebSocketMessage)
          );
          break;

        case 'subscribe':
          // Handle subscription to specific data updates
          handleSubscription(ws, data);
          break;

        case 'unsubscribe':
          // Handle unsubscription
          handleUnsubscription(ws, data);
          break;

        default:
          console.warn('Unknown WebSocket message type:', data.type);
      }
    } catch (error) {
      console.error('WebSocket message parsing error:', error);
      ws.send(
        JSON.stringify({
          type: 'error',
          timestamp: new Date().toISOString(),
          data: {
            message: 'Invalid message format',
            code: 'PARSE_ERROR',
          },
        } satisfies WebSocketMessage)
      );
    }
  },

  open(ws: ServerWebSocket<WebSocketData>) {
    const clientId = crypto.randomUUID();
    ws.data = {
      id: clientId,
      connectedAt: new Date(),
    };

    connections.set(clientId, ws);

    console.log('WebSocket connection opened:', {
      clientId,
      totalConnections: connections.size,
    });

    // Send welcome message
    ws.send(
      JSON.stringify({
        type: 'connected',
        timestamp: new Date().toISOString(),
        data: {
          clientId,
          serverVersion: '0.1.0',
        },
      } satisfies WebSocketMessage)
    );
  },

  close(ws: ServerWebSocket<WebSocketData>) {
    if (ws.data?.id) {
      connections.delete(ws.data.id);
      console.log('WebSocket connection closed:', {
        clientId: ws.data.id,
        totalConnections: connections.size,
      });
    }
  },

  error(ws: ServerWebSocket<WebSocketData>, error: Error) {
    console.error('WebSocket error:', {
      clientId: ws.data?.id,
      error: error.message,
      stack: error.stack,
    });
  },
};

function handleSubscription(
  ws: ServerWebSocket<WebSocketData>,
  message: WebSocketMessage
) {
  // Handle subscription logic here
  // For now, just acknowledge the subscription
  ws.send(
    JSON.stringify({
      type: 'subscribed',
      timestamp: new Date().toISOString(),
      data: {
        subscription: message.data,
      },
    } satisfies WebSocketMessage)
  );
}

function handleUnsubscription(
  ws: ServerWebSocket<WebSocketData>,
  message: WebSocketMessage
) {
  // Handle unsubscription logic here
  ws.send(
    JSON.stringify({
      type: 'unsubscribed',
      timestamp: new Date().toISOString(),
      data: {
        subscription: message.data,
      },
    } satisfies WebSocketMessage)
  );
}

export function broadcastToAll(message: WebSocketMessage): void {
  const messageStr = JSON.stringify(message);
  connections.forEach(ws => {
    try {
      ws.send(messageStr);
    } catch (error) {
      console.error('Failed to send message to client:', ws.data?.id, error);
    }
  });
}

export function sendToClient(
  clientId: string,
  message: WebSocketMessage
): boolean {
  const ws = connections.get(clientId);
  if (ws) {
    try {
      ws.send(JSON.stringify(message));
      return true;
    } catch (error) {
      console.error('Failed to send message to client:', clientId, error);
    }
  }
  return false;
}

export function getConnectedClients(): string[] {
  return Array.from(connections.keys());
}

export function getConnectionCount(): number {
  return connections.size;
}
