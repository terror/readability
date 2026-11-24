import type { Meta, StoryObj } from '@storybook/react-vite';

import { IconButton } from './icon-button';
import { CodeIcon } from './icons';

const meta = {
  args: { children: <CodeIcon />, label: 'Source' },
  component: IconButton,
  title: 'UI/Icon button',
} satisfies Meta<typeof IconButton>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Active: Story = { args: { active: true } };
