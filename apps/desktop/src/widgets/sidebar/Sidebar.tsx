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
  IconCalendarEvent,
} from '@tabler/icons-react';
import { useNavigate, useLocation } from 'react-router-dom';

import { ROUTES } from '../../app/routes/routes';

interface NavItem {
  label: string;
  icon: React.ReactNode;
  path?: string;
  onClick?: () => void;
}

export function Sidebar() {
  const [searchValue, setSearchValue] = useState('');
  const navigate = useNavigate();
  const location = useLocation();

  const mainNavItems: NavItem[] = [
    {
      label: 'Daily View',
      icon: <IconCalendarEvent size={20} />,
      path: ROUTES.DAILY_VIEW,
    },
    {
      label: 'Calendar',
      icon: <IconCalendar size={20} />,
      path: ROUTES.CALENDAR_VIEW,
    },
    {
      label: 'Hierarchy',
      icon: <IconHierarchy3 size={20} />,
      path: ROUTES.HIERARCHY_VIEW,
    },
  ];

  const organizationItems: NavItem[] = [
    {
      label: 'Life Areas',
      icon: <IconFolder size={20} />,
      onClick: () => {/* TODO: Implement areas list view */},
    },
    {
      label: 'Goals',
      icon: <IconTarget size={20} />,
      onClick: () => {/* TODO: Implement goals list view */},
    },
    {
      label: 'Projects',
      icon: <IconFolder size={20} />,
      onClick: () => {/* TODO: Implement projects list view */},
    },
    {
      label: 'Tasks',
      icon: <IconCheckbox size={20} />,
      onClick: () => {/* TODO: Implement tasks list view */},
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
      path: ROUTES.SETTINGS,
    },
  ];

  const handleSearch = () => {
    if (searchValue.trim()) {
      void navigate(`${ROUTES.SEARCH}?q=${encodeURIComponent(searchValue)}`);
    }
  };

  return (
    <Stack gap={0} h="100%">
      <Box mb="md">
        <TextInput
          className="mb-4"
          leftSection={<IconSearch size={16} />}
          placeholder="Search..."
          value={searchValue}
          onChange={(event) => setSearchValue(event.currentTarget.value)}
          onKeyDown={(event) => {
            if (event.key === 'Enter') {
              handleSearch();
            }
          }}
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
                  active={item.path === location.pathname}
                  className="rounded-md"
                  label={item.label}
                  leftSection={item.icon}
                  onClick={() => {
                    if (item.path) {void navigate(item.path);}
                    item.onClick?.();
                  }}
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
                  onClick={() => {
                    if (item.path) {void navigate(item.path);}
                    item.onClick?.();
                  }}
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
            active={item.path === location.pathname}
            className="rounded-md"
            label={item.label}
            leftSection={item.icon}
            onClick={() => {
              if (item.path) {void navigate(item.path);}
              item.onClick?.();
            }}
          />
        ))}
      </Stack>
    </Stack>
  );
}