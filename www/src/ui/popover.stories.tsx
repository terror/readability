import type { Meta, StoryObj } from '@storybook/react-vite';

import { InfoIcon } from './icons';
import { Popover } from './popover';

const meta = {
  args: {
    children: <p className='text-ink-2 text-sm'>Popover content.</p>,
    icon: <InfoIcon />,
    label: 'Details',
  },
  component: Popover,
  title: 'UI/Popover',
} satisfies Meta<typeof Popover>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
