import { IconButton } from '@/ui/icon-button';
import { InfoIcon } from '@/ui/icons';
import type { Meta, StoryObj } from '@storybook/react-vite';

import { TopBar } from './top-bar';

const meta = {
  args: {
    actions: (
      <>
        <IconButton label='Appearance'>
          <span className='font-read text-[15px] font-medium'>Aa</span>
        </IconButton>
        <IconButton label='Details'>
          <InfoIcon />
        </IconButton>
      </>
    ),
  },
  component: TopBar,
  parameters: { layout: 'fullscreen' },
  title: 'App/Top bar',
} satisfies Meta<typeof TopBar>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
