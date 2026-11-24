import type { Meta, StoryObj } from '@storybook/react-vite';

import { Switch } from './switch';

const meta = {
  args: { 'aria-label': 'Read JSON-LD metadata' },
  component: Switch,
  title: 'UI/Switch',
} satisfies Meta<typeof Switch>;

export default meta;

type Story = StoryObj<typeof meta>;

export const On: Story = { args: { defaultChecked: true } };

export const Off: Story = {};

export const Disabled: Story = { args: { disabled: true } };
