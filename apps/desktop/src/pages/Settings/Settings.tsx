import { useState } from 'react';

import { Tabs, Title, Paper, Group, Text, Button, Stack, Card, Alert } from '@mantine/core';
import {
  IconSettings,
  IconBrush,
  IconDatabase,
  IconPlug,
  IconCheck,
  IconX,
} from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/core';

function Settings() {
  const [databaseTestResult, setDatabaseTestResult] = useState<string | null>(null);
  const [isTestingDatabase, setIsTestingDatabase] = useState(false);
  const [testError, setTestError] = useState<string | null>(null);

  const testDatabase = async () => {
    setIsTestingDatabase(true);
    setTestError(null);
    setDatabaseTestResult(null);

    try {
      const result = await invoke<string>('test_database');
      setDatabaseTestResult(result);
    } catch (error) {
      setTestError(error as string);
    } finally {
      setIsTestingDatabase(false);
    }
  };

  return (
    <div className="p-6">
      <Title className="mb-6" order={1}>
        Settings
      </Title>

      <Tabs defaultValue="general">
        <Tabs.List>
          <Tabs.Tab leftSection={<IconSettings size={16} />} value="general">
            General
          </Tabs.Tab>
          <Tabs.Tab leftSection={<IconBrush size={16} />} value="appearance">
            Appearance
          </Tabs.Tab>
          <Tabs.Tab leftSection={<IconDatabase size={16} />} value="data">
            Data & Storage
          </Tabs.Tab>
          <Tabs.Tab leftSection={<IconPlug size={16} />} value="plugins">
            Plugins
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel pt="xs" value="general">
          <Paper mt="md" p="md">
            <Text>General settings will be implemented in Phase 2.</Text>
          </Paper>
        </Tabs.Panel>

        <Tabs.Panel pt="xs" value="appearance">
          <Paper mt="md" p="md">
            <Text>
              Appearance settings including theme selection will be implemented in Phase 3.
            </Text>
          </Paper>
        </Tabs.Panel>

        <Tabs.Panel pt="xs" value="data">
          <Stack gap="md" mt="md">
            <Card padding="lg" shadow="sm">
              <Title className="mb-4" order={3}>
                Database Status
              </Title>

              <Group>
                <Button
                  leftSection={<IconDatabase size={16} />}
                  loading={isTestingDatabase}
                  onClick={() => void testDatabase()}
                >
                  Test Database Connection
                </Button>
              </Group>

              {databaseTestResult && (
                <Alert
                  color="green"
                  icon={<IconCheck size={16} />}
                  mt="md"
                  title="Database Test Successful"
                >
                  <Text size="sm" style={{ whiteSpace: 'pre-line' }}>
                    {databaseTestResult}
                  </Text>
                </Alert>
              )}

              {testError && (
                <Alert color="red" icon={<IconX size={16} />} mt="md" title="Database Test Failed">
                  <Text size="sm">{testError}</Text>
                </Alert>
              )}
            </Card>

            <Card padding="lg" shadow="sm">
              <Title className="mb-4" order={3}>
                Data Management
              </Title>
              <Text>Import/export functionality will be implemented in Phase 2.</Text>
            </Card>
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel pt="xs" value="plugins">
          <Paper mt="md" p="md">
            <Text>Plugin management will be implemented in Phase 4.</Text>
          </Paper>
        </Tabs.Panel>
      </Tabs>
    </div>
  );
}

export default Settings;
