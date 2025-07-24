import { Container, Title, Text, Card, Stack, Tabs, Switch, Select , useMantineColorScheme } from '@mantine/core';
import { IconPalette, IconSettings, IconDatabase, IconPlug } from '@tabler/icons-react';

export default function SettingsPage() {
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();

  return (
    <Container py="xl" size="lg">
      <Stack gap="xl">
        <div>
          <Title order={1}>Settings</Title>
          <Text c="dimmed" size="lg">
            Configure your EvorBrain experience
          </Text>
        </div>

        <Tabs defaultValue="appearance">
          <Tabs.List>
            <Tabs.Tab leftSection={<IconPalette size={16} />} value="appearance">
              Appearance
            </Tabs.Tab>
            <Tabs.Tab leftSection={<IconSettings size={16} />} value="general">
              General
            </Tabs.Tab>
            <Tabs.Tab leftSection={<IconDatabase size={16} />} value="data">
              Data & Storage
            </Tabs.Tab>
            <Tabs.Tab leftSection={<IconPlug size={16} />} value="plugins">
              Plugins
            </Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel pt="xs" value="appearance">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Stack gap="md">
                  <Switch
                    checked={colorScheme === 'dark'}
                    description="Toggle between light and dark themes"
                    label="Dark Mode"
                    size="md"
                    onChange={() => toggleColorScheme()}
                  />
                  
                  <Select
                    data={[
                      { value: 'small', label: 'Small' },
                      { value: 'medium', label: 'Medium' },
                      { value: 'large', label: 'Large' },
                    ]}
                    defaultValue="medium"
                    description="Choose your preferred font size"
                    label="Font Size"
                  />
                </Stack>
              </Card>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel pt="xs" value="general">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Stack gap="md">
                  <Switch
                    defaultChecked
                    description="Automatically save changes as you type"
                    label="Auto-save"
                    size="md"
                  />
                  
                  <Select
                    data={[
                      { value: 'daily', label: 'Daily View' },
                      { value: 'calendar', label: 'Calendar View' },
                      { value: 'hierarchy', label: 'Hierarchy View' },
                    ]}
                    defaultValue="daily"
                    description="Choose which view to show on startup"
                    label="Default View"
                  />
                </Stack>
              </Card>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel pt="xs" value="data">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Stack gap="md">
                  <Text fw={500}>Data Location</Text>
                  <Text c="dimmed" size="sm">~/Documents/EvorBrain</Text>
                  
                  <Text fw={500} mt="md">Backup Settings</Text>
                  <Switch
                    defaultChecked
                    description="Create daily backups of your data"
                    label="Automatic Backups"
                    size="md"
                  />
                </Stack>
              </Card>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel pt="xs" value="plugins">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Text c="dimmed" py="xl" ta="center">
                  Plugin system will be available in Phase 4
                </Text>
              </Card>
            </Stack>
          </Tabs.Panel>
        </Tabs>
      </Stack>
    </Container>
  );
}