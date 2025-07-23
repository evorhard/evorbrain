import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { appWindow } from '@tauri-apps/api/window';
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
import { Container, Title, Text, Card, Group, Button, Stack, Paper, Badge, Space } from '@mantine/core';
import { notifications } from '@mantine/notifications';

export function App() {
  const [greeting, setGreeting] = useState('');
  const [appInfo, setAppInfo] = useState<{
    name: string;
    version: string;
    tauriVersion: string;
  } | null>(null);
  const [windowTitle, setWindowTitle] = useState('EvorBrain');

  useEffect(() => {
    // Load app information
    Promise.all([getName(), getVersion(), getTauriVersion()]).then(
      ([name, version, tauriVersion]) => {
        setAppInfo({ name, version, tauriVersion });
      }
    );

    // Test window events
    const unlistenPromise = appWindow.listen('tauri://resize', () => {
      // Window resize event handler
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const handleGreet = async () => {
    try {
      const response = await invoke<string>('greet', { name: 'User' });
      setGreeting(response);
      notifications.show({
        title: 'Success',
        message: 'IPC communication working!',
        color: 'green',
      });
    } catch (error) {
      setGreeting('Error: Failed to call greet command');
      notifications.show({
        title: 'Error',
        message: 'Failed to call greet command',
        color: 'red',
      });
    }
  };

  const handleUpdateTitle = async () => {
    const newTitle = `EvorBrain - ${new Date().toLocaleTimeString()}`;
    await appWindow.setTitle(newTitle);
    setWindowTitle(newTitle);
  };

  const handleMinimize = () => appWindow.minimize();
  const handleMaximize = () => appWindow.toggleMaximize();
  const handleCenter = () => appWindow.center();

  return (
    <Container size="md" py="xl">
      <Stack spacing="xl">
        <div>
          <Title order={1} className="flex items-center gap-2">
            <span>🧠</span> EvorBrain
          </Title>
          <Text size="lg" c="dimmed">Personal Knowledge Management System</Text>
        </div>

        {appInfo && (
          <Card shadow="sm" padding="lg" radius="md" withBorder>
            <Title order={3} size="h4" mb="md">App Information</Title>
            <Stack spacing="xs">
              <Group>
                <Text fw={500}>Name:</Text>
                <Text>{appInfo.name}</Text>
              </Group>
              <Group>
                <Text fw={500}>Version:</Text>
                <Badge color="blue" variant="light">{appInfo.version}</Badge>
              </Group>
              <Group>
                <Text fw={500}>Tauri Version:</Text>
                <Badge color="gray" variant="light">{appInfo.tauriVersion}</Badge>
              </Group>
            </Stack>
          </Card>
        )}

        <Paper shadow="xs" p="lg" radius="md" withBorder>
          <Title order={3} size="h4" mb="md">Test IPC Communication</Title>
          <Button onClick={handleGreet} variant="filled">
            Test Greet Command
          </Button>
          {greeting && (
            <Text c="green" mt="md">{greeting}</Text>
          )}
        </Paper>

        <Paper shadow="xs" p="lg" radius="md" withBorder>
          <Title order={3} size="h4" mb="md">Window Controls</Title>
          <Group>
            <Button onClick={handleUpdateTitle} variant="default">
              Update Title
            </Button>
            <Button onClick={handleMinimize} variant="default">
              Minimize
            </Button>
            <Button onClick={handleMaximize} variant="default">
              Toggle Maximize
            </Button>
            <Button onClick={handleCenter} variant="default">
              Center Window
            </Button>
          </Group>
          <Text size="sm" c="dimmed" mt="md">Current Title: {windowTitle}</Text>
        </Paper>

        <Card shadow="sm" padding="lg" radius="md" withBorder bg="teal.0">
          <Stack spacing="xs">
            <Group spacing="xs">
              <Text size="sm" c="teal">✅</Text>
              <Text size="sm">Tauri is working correctly!</Text>
            </Group>
            <Group spacing="xs">
              <Text size="sm" c="teal">✅</Text>
              <Text size="sm">Window controls are functional</Text>
            </Group>
            <Group spacing="xs">
              <Text size="sm" c="teal">✅</Text>
              <Text size="sm">IPC communication is established</Text>
            </Group>
            <Group spacing="xs">
              <Text size="sm" c="teal">✅</Text>
              <Text size="sm">Mantine UI is configured</Text>
            </Group>
          </Stack>
        </Card>
      </Stack>
    </Container>
  );
}
