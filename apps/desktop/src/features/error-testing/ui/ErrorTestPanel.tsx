import { useState } from 'react';

import { Button, Card, Stack, Text, Title, Alert, Code, Group, Badge } from '@mantine/core';
import { IconAlertCircle, IconInfoCircle, IconAlertTriangle, IconX } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';

interface ErrorResponse {
  code: number;
  message: string;
  user_message: string;
  severity: 'Info' | 'Warning' | 'Error' | 'Critical';
  recoverable: boolean;
  suggestions: string[];
  help_url?: string;
}

export function ErrorTestPanel() {
  const [lastError, setLastError] = useState<ErrorResponse | null>(null);
  const [loading, setLoading] = useState(false);

  const testError = async (errorType: string) => {
    setLoading(true);
    try {
      await invoke<string>('test_error_handling', { errorType });
      setLastError(null);
      // Success case logged
    } catch (error: unknown) {
      // The error is serialized as a structured object
      setLastError(error as ErrorResponse);
    } finally {
      setLoading(false);
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'Info':
        return 'blue';
      case 'Warning':
        return 'yellow';
      case 'Error':
        return 'red';
      case 'Critical':
        return 'red';
      default:
        return 'gray';
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'Info':
        return <IconInfoCircle size={20} />;
      case 'Warning':
        return <IconAlertTriangle size={20} />;
      case 'Error':
        return <IconAlertCircle size={20} />;
      case 'Critical':
        return <IconX size={20} />;
      default:
        return null;
    }
  };

  return (
    <Card>
      <Title mb="md" order={3}>
        Error Handling Test Panel
      </Title>

      <Text c="dimmed" mb="lg" size="sm">
        Test different error scenarios to see how the application handles them.
      </Text>

      <Stack gap="sm">
        <Group>
          <Button
            color="orange"
            loading={loading}
            variant="light"
            onClick={() => void testError('validation')}
          >
            Test Validation Error
          </Button>

          <Button
            color="yellow"
            loading={loading}
            variant="light"
            onClick={() => void testError('not_found')}
          >
            Test Not Found Error
          </Button>

          <Button
            color="red"
            loading={loading}
            variant="light"
            onClick={() => void testError('database')}
          >
            Test Database Error
          </Button>

          <Button
            color="pink"
            loading={loading}
            variant="light"
            onClick={() => void testError('file')}
          >
            Test File Error
          </Button>

          <Button
            color="green"
            loading={loading}
            variant="light"
            onClick={() => void testError('success')}
          >
            Test Success
          </Button>
        </Group>

        {lastError && (
          <Alert
            color={getSeverityColor(lastError.severity)}
            icon={getSeverityIcon(lastError.severity)}
            mt="md"
            title={
              <Group gap="xs">
                <Text>Error {lastError.code}</Text>
                <Badge color={getSeverityColor(lastError.severity)} size="sm">
                  {lastError.severity}
                </Badge>
                {lastError.recoverable && (
                  <Badge color="green" size="sm">
                    Recoverable
                  </Badge>
                )}
              </Group>
            }
          >
            <Stack gap="sm">
              <Text fw={500}>{lastError.user_message}</Text>

              {lastError.suggestions.length > 0 && (
                <div>
                  <Text fw={500} size="sm">
                    Suggestions:
                  </Text>
                  <ul style={{ margin: '4px 0', paddingLeft: '20px' }}>
                    {lastError.suggestions.map((suggestion, index) => (
                      <li key={index}>
                        <Text size="sm">{suggestion}</Text>
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {lastError.help_url && (
                <Text size="sm">
                  <a href={lastError.help_url} rel="noopener noreferrer" target="_blank">
                    Learn more →
                  </a>
                </Text>
              )}

              <details>
                <summary style={{ cursor: 'pointer' }}>
                  <Text c="dimmed" component="span" size="sm">
                    Technical details
                  </Text>
                </summary>
                <Code block mt="xs">
                  {lastError.message}
                </Code>
              </details>
            </Stack>
          </Alert>
        )}
      </Stack>
    </Card>
  );
}
