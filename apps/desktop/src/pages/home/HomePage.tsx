import { Container, Title, Text, Card, Stack, Button } from '@mantine/core';
import { IconArrowRight } from '@tabler/icons-react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '../../app/routes/routes';

export default function HomePage() {
  const navigate = useNavigate();

  return (
    <Container py="xl" size="md">
      <Stack gap="xl">
        <div>
          <Title className="flex items-center gap-2" order={1}>
            <span>🧠</span> Welcome to EvorBrain
          </Title>
          <Text c="dimmed" size="lg">
            Your personal knowledge management system
          </Text>
        </div>

        <Stack gap="md">
          <Card withBorder padding="lg" radius="md" shadow="sm">
            <Title mb="md" order={3} size="h4">
              Quick Start
            </Title>
            <Stack gap="sm">
              <Button
                rightSection={<IconArrowRight size={16} />}
                variant="subtle"
                onClick={() => void navigate(ROUTES.DAILY_VIEW)}
              >
                Go to Daily View
              </Button>
              <Button
                rightSection={<IconArrowRight size={16} />}
                variant="subtle"
                onClick={() => void navigate(ROUTES.HIERARCHY_VIEW)}
              >
                Browse Hierarchy
              </Button>
            </Stack>
          </Card>

          <Card withBorder padding="lg" radius="md" shadow="sm">
            <Title mb="md" order={3} size="h4">
              Recent Activity
            </Title>
            <Text c="dimmed">No recent activity to show</Text>
          </Card>
        </Stack>
      </Stack>
    </Container>
  );
}