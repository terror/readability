import type { Meta, StoryObj } from '@storybook/react-vite';

import { Button } from './button';

const meta = {
  args: { children: 'Read' },
  component: Button,
  title: 'UI/Button',
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Primary: Story = { args: { variant: 'primary' } };

export const Secondary: Story = { args: { children: 'Paste HTML' } };

export const Quiet: Story = { args: { children: 'Reset', variant: 'quiet' } };

export const Disabled: Story = { args: { disabled: true, variant: 'primary' } };
