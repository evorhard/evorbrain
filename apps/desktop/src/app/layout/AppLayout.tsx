import type { ReactNode } from 'react';

import { AppShell, Group, Text, ActionIcon, useMantineColorScheme, Burger } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IconSun, IconMoon, IconBrain } from '@tabler/icons-react';

import { Sidebar } from '../../widgets/sidebar/Sidebar';

interface AppLayoutProps {
  children: ReactNode;
}

export function AppLayout({ children }: AppLayoutProps) {
  const [opened, { toggle }] = useDisclosure();
  const { colorScheme, toggleColorScheme } = useMantineColorScheme();

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: 'sm',
        collapsed: { mobile: !opened },
      }}
      padding="md"
    >
      <AppShell.Header className="border-b border-gray-200 dark:border-gray-800">
        <Group h="100%" justify="space-between" px="md">
          <Group>
            <Burger hiddenFrom="sm" opened={opened} size="sm" onClick={toggle} />
            <Group gap="xs">
              <IconBrain className="text-teal-600 dark:text-teal-400" size={28} />
              <Text className="select-none" fw={700} size="xl">
                EvorBrain
              </Text>
            </Group>
          </Group>

          <ActionIcon
            aria-label="Toggle color scheme"
            size="lg"
            variant="default"
            onClick={() => toggleColorScheme()}
          >
            {colorScheme === 'dark' ? <IconSun size={18} /> : <IconMoon size={18} />}
          </ActionIcon>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar p="md">
        <Sidebar />
      </AppShell.Navbar>

      <AppShell.Main className="bg-gray-50 dark:bg-gray-900">{children}</AppShell.Main>
    </AppShell>
  );
}
