import { Container, Title, Text, Card, Stack, Tree } from '@mantine/core';
import { IconFolder, IconTarget, IconListCheck, IconCircleCheck } from '@tabler/icons-react';

// Sample data for demonstration
const data = [
  {
    value: 'area-1',
    label: 'Personal Development',
    nodeProps: {
      icon: <IconFolder size={16} />,
    },
    children: [
      {
        value: 'goal-1',
        label: 'Learn Programming',
        nodeProps: {
          icon: <IconTarget size={16} />,
        },
        children: [
          {
            value: 'project-1',
            label: 'Build EvorBrain',
            nodeProps: {
              icon: <IconListCheck size={16} />,
            },
            children: [
              {
                value: 'task-1',
                label: 'Set up project structure',
                nodeProps: {
                  icon: <IconCircleCheck size={16} />,
                },
              },
              {
                value: 'task-2',
                label: 'Implement routing',
                nodeProps: {
                  icon: <IconCircleCheck size={16} />,
                },
              },
            ],
          },
        ],
      },
    ],
  },
  {
    value: 'area-2',
    label: 'Health & Fitness',
    nodeProps: {
      icon: <IconFolder size={16} />,
    },
    children: [
      {
        value: 'goal-2',
        label: 'Improve Physical Health',
        nodeProps: {
          icon: <IconTarget size={16} />,
        },
      },
    ],
  },
];

export default function HierarchyViewPage() {
  return (
    <Container py="xl" size="lg">
      <Stack gap="xl">
        <div>
          <Title order={1}>Hierarchy View</Title>
          <Text c="dimmed" size="lg">
            Browse your areas, goals, projects, and tasks
          </Text>
        </div>

        <Card withBorder padding="lg" radius="md" shadow="sm">
          <Tree expandOnClick data={data} levelOffset={23} />
        </Card>

        <Card withBorder bg="blue.0" padding="sm" radius="md" shadow="xs">
          <Text c="blue.7" size="sm">
            <strong>Note:</strong> This is sample data. The actual hierarchy will be loaded from
            your markdown files once the backend is connected.
          </Text>
        </Card>
      </Stack>
    </Container>
  );
}
