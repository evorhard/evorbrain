import { Container, Title, Text, Card, Stack } from '@mantine/core';

export default function CalendarViewPage() {
  return (
    <Container py="xl" size="lg">
      <Stack gap="xl">
        <div>
          <Title order={1}>Calendar View</Title>
          <Text c="dimmed" size="lg">
            View and manage your tasks on a calendar
          </Text>
        </div>

        <Card withBorder padding="lg" radius="md" shadow="sm">
          <Text c="dimmed" py="xl" ta="center">
            Calendar component will be implemented in Phase 3 with FullCalendar integration
          </Text>
        </Card>
      </Stack>
    </Container>
  );
}
