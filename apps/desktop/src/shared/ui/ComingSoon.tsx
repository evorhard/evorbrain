import { Container, Title, Text, Button, Stack } from '@mantine/core';
import { IconArrowLeft } from '@tabler/icons-react';
import { useNavigate } from 'react-router-dom';

interface ComingSoonProps {
  title: string;
  description?: string;
}

export function ComingSoon({ title, description }: ComingSoonProps) {
  const navigate = useNavigate();

  return (
    <Container className="flex h-full items-center justify-center" size="sm">
      <Stack align="center" className="text-center">
        <Title className="text-4xl" order={1}>
          {title}
        </Title>
        <Text c="dimmed" className="mb-4" size="lg">
          {description || 'This feature is coming soon. Check back later!'}
        </Text>
        <Button
          leftSection={<IconArrowLeft size={16} />}
          variant="subtle"
          onClick={() => {
            void navigate(-1);
          }}
        >
          Go Back
        </Button>
      </Stack>
    </Container>
  );
}
