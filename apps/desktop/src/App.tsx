import React, { useState, useEffect } from 'react';

import { Container, Title, Text, Card, Group, Button, Stack, Paper, Badge } from '@mantine/core';
import { notifications } from '@mantine/notifications';
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

import { AppLayout } from './app/layout';

export function App() {
  const appWindow = getCurrentWindow();
  const [greeting, setGreeting] = useState('');
  const [appInfo, setAppInfo] = useState<{
    name: string;
    version: string;
    tauriVersion: string;
  } | null>(null);
  const [windowTitle, setWindowTitle] = useState('EvorBrain');

  useEffect(() => {
    // Load app information
    void Promise.all([getName(), getVersion(), getTauriVersion()]).then(
      ([name, version, tauriVersion]) => {
        setAppInfo({ name, version, tauriVersion });
      }
    );

    // Test window events
    const unlistenPromise = appWindow.listen('tauri://resize', () => {
      // Window resize event handler
    });

    return () => {
      void unlistenPromise.then((unlisten: () => void) => unlisten());
    };
  }, [appWindow]);

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

  const handleUpdateTitle = () => {
    const newTitle = `EvorBrain - ${new Date().toLocaleTimeString()}`;
    void appWindow.setTitle(newTitle);
    setWindowTitle(newTitle);
  };

  const handleMinimize = () => void appWindow.minimize();
  const handleMaximize = () => void appWindow.toggleMaximize();
  const handleCenter = () => void appWindow.center();

  return (
    <AppLayout>
      <Container py="xl" size="md">
        <Stack gap="xl">
        <div>
          <Title className="flex items-center gap-2" order={1}>
            <span>🧠</span> EvorBrain
          </Title>
          <Text c="dimmed" size="lg">Personal Knowledge Management System</Text>
        </div>

        {appInfo && (
          <Card withBorder padding="lg" radius="md" shadow="sm">
            <Title mb="md" order={3} size="h4">App Information</Title>
            <Stack gap="xs">
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

        <Paper withBorder p="lg" radius="md" shadow="xs">
          <Title mb="md" order={3} size="h4">Test IPC Communication</Title>
          <Button variant="filled" onClick={() => void handleGreet()}>
            Test Greet Command
          </Button>
          {greeting && (
            <Text c="green" mt="md">{greeting}</Text>
          )}
        </Paper>

        <Paper withBorder p="lg" radius="md" shadow="xs">
          <Title mb="md" order={3} size="h4">Window Controls</Title>
          <Group>
            <Button variant="default" onClick={handleUpdateTitle}>
              Update Title
            </Button>
            <Button variant="default" onClick={handleMinimize}>
              Minimize
            </Button>
            <Button variant="default" onClick={handleMaximize}>
              Toggle Maximize
            </Button>
            <Button variant="default" onClick={handleCenter}>
              Center Window
            </Button>
          </Group>
          <Text c="dimmed" mt="md" size="sm">Current Title: {windowTitle}</Text>
        </Paper>

        <Card withBorder bg="teal.0" padding="lg" radius="md" shadow="sm">
          <Stack gap="xs">
            <Group gap="xs">
              <Text c="teal" size="sm">✅</Text>
              <Text size="sm">Tauri is working correctly!</Text>
            </Group>
            <Group gap="xs">
              <Text c="teal" size="sm">✅</Text>
              <Text size="sm">Window controls are functional</Text>
            </Group>
            <Group gap="xs">
              <Text c="teal" size="sm">✅</Text>
              <Text size="sm">IPC communication is established</Text>
            </Group>
            <Group gap="xs">
              <Text c="teal" size="sm">✅</Text>
              <Text size="sm">Mantine UI is configured</Text>
            </Group>
          </Stack>
        </Card>
        </Stack>
      </Container>
    </AppLayout>
  );
}
