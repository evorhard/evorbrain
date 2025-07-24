import { useState } from 'react';

import { Stack, NavLink, TextInput, ScrollArea, Box, Divider, Text } from '@mantine/core';
import {
  IconSearch,
  IconCalendar,
  IconHierarchy3,
  IconTarget,
  IconFolder,
  IconCheckbox,
  IconCommand,
  IconSettings,
} from '@tabler/icons-react';

interface NavItem {
  label: string;
  icon: React.ReactNode;
  onClick?: () => void;
  active?: boolean;
}

export function Sidebar() {
  const [searchValue, setSearchValue] = useState('');
  const [activeView, setActiveView] = useState('daily');

  const mainNavItems: NavItem[] = [
    {
      label: 'Daily View',
      icon: <IconCalendar size={20} />,
      active: activeView === 'daily',
      onClick: () => setActiveView('daily'),
    },
    {
      label: 'Calendar',
      icon: <IconCalendar size={20} />,
      active: activeView === 'calendar',
      onClick: () => setActiveView('calendar'),
    },
    {
      label: 'Hierarchy',
      icon: <IconHierarchy3 size={20} />,
      active: activeView === 'hierarchy',
      onClick: () => setActiveView('hierarchy'),
    },
  ];

  const organizationItems: NavItem[] = [
    {
      label: 'Life Areas',
      icon: <IconFolder size={20} />,
      onClick: () => setActiveView('areas'),
    },
    {
      label: 'Goals',
      icon: <IconTarget size={20} />,
      onClick: () => setActiveView('goals'),
    },
    {
      label: 'Projects',
      icon: <IconFolder size={20} />,
      onClick: () => setActiveView('projects'),
    },
    {
      label: 'Tasks',
      icon: <IconCheckbox size={20} />,
      onClick: () => setActiveView('tasks'),
    },
  ];

  const bottomItems: NavItem[] = [
    {
      label: 'Command Palette',
      icon: <IconCommand size={20} />,
      onClick: () => {/* TODO: Open command palette */},
    },
    {
      label: 'Settings',
      icon: <IconSettings size={20} />,
      onClick: () => {/* TODO: Open settings */},
    },
  ];

  return (
    <Stack gap={0} h="100%">
      <Box mb="md">
        <TextInput
          className="mb-4"
          leftSection={<IconSearch size={16} />}
          placeholder="Search..."
          value={searchValue}
          onChange={(event) => setSearchValue(event.currentTarget.value)}
        />
      </Box>

      <ScrollArea offsetScrollbars flex={1}>
        <Stack gap="lg">
          <Box>
            <Text c="dimmed" className="uppercase tracking-wider" fw={600} mb="xs" size="xs">
              Views
            </Text>
            <Stack gap={4}>
              {mainNavItems.map((item) => (
                <NavLink
                  key={item.label}
                  active={item.active}
                  className="rounded-md"
                  label={item.label}
                  leftSection={item.icon}
                  onClick={item.onClick}
                />
              ))}
            </Stack>
          </Box>

          <Divider />

          <Box>
            <Text c="dimmed" className="uppercase tracking-wider" fw={600} mb="xs" size="xs">
              Organization
            </Text>
            <Stack gap={4}>
              {organizationItems.map((item) => (
                <NavLink
                  key={item.label}
                  className="rounded-md"
                  label={item.label}
                  leftSection={item.icon}
                  onClick={item.onClick}
                />
              ))}
            </Stack>
          </Box>
        </Stack>
      </ScrollArea>

      <Divider my="md" />

      <Stack gap={4}>
        {bottomItems.map((item) => (
          <NavLink
            key={item.label}
            className="rounded-md"
            label={item.label}
            leftSection={item.icon}
            onClick={item.onClick}
          />
        ))}
      </Stack>
    </Stack>
  );
}