import type { Meta, StoryObj } from '@storybook/react-vite';

import { Status } from './status';

const meta = {
  args: { children: 'Readable', tone: 'ok' },
  component: Status,
  title: 'UI/Status',
} satisfies Meta<typeof Status>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Ok: Story = {};

export const Warn: Story = {
  args: { children: 'Probably not an article', tone: 'warn' },
};

export const Danger: Story = {
  args: { children: 'Could not load', tone: 'danger' },
};
