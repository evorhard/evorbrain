import { Container, Title, Text, Tabs, Card, Stack, Badge, Group } from '@mantine/core';
import { IconCalendarEvent, IconClock, IconCheck } from '@tabler/icons-react';

export default function DailyViewPage() {
  return (
    <Container py="xl" size="lg">
      <Stack gap="xl">
        <div>
          <Title order={1}>Daily View</Title>
          <Text c="dimmed" size="lg">
            {new Date().toLocaleDateString('en-US', {
              weekday: 'long',
              year: 'numeric',
              month: 'long',
              day: 'numeric',
            })}
          </Text>
        </div>

        <Tabs defaultValue="today">
          <Tabs.List>
            <Tabs.Tab leftSection={<IconCalendarEvent size={16} />} value="today">
              Today
            </Tabs.Tab>
            <Tabs.Tab leftSection={<IconClock size={16} />} value="upcoming">
              Upcoming
            </Tabs.Tab>
            <Tabs.Tab leftSection={<IconCheck size={16} />} value="completed">
              Completed
            </Tabs.Tab>
          </Tabs.List>

          <Tabs.Panel pt="xs" value="today">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Group justify="space-between" mb="xs">
                  <Text fw={500}>Sample Task</Text>
                  <Badge color="blue" variant="light">
                    In Progress
                  </Badge>
                </Group>
                <Text c="dimmed" size="sm">
                  This is a placeholder task. Tasks will be loaded from the backend.
                </Text>
              </Card>

              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Text c="dimmed" ta="center">
                  No more tasks for today
                </Text>
              </Card>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel pt="xs" value="upcoming">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Text c="dimmed" ta="center">
                  No upcoming tasks
                </Text>
              </Card>
            </Stack>
          </Tabs.Panel>

          <Tabs.Panel pt="xs" value="completed">
            <Stack gap="md" mt="md">
              <Card withBorder padding="lg" radius="md" shadow="sm">
                <Text c="dimmed" ta="center">
                  No completed tasks today
                </Text>
              </Card>
            </Stack>
          </Tabs.Panel>
        </Tabs>
      </Stack>
    </Container>
  );
}
