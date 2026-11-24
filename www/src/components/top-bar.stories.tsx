import { IconButton } from '@/ui/icon-button';
import { CodeIcon, InfoIcon } from '@/ui/icons';
import { LinkField } from '@/ui/link-field';
import type { Meta, StoryObj } from '@storybook/react-vite';

import { TopBar } from './top-bar';

const meta = {
  args: {
    actions: (
      <>
        <IconButton label='Source'>
          <CodeIcon />
        </IconButton>
        <IconButton label='Appearance'>
          <span className='font-read text-[15px] font-medium'>Aa</span>
        </IconButton>
        <IconButton label='Details'>
          <InfoIcon />
        </IconButton>
      </>
    ),
    field: (
      <LinkField
        value='harborledger.example/news/riverside-tram-line'
        onChange={() => {}}
        onSubmit={() => {}}
      />
    ),
  },
  component: TopBar,
  parameters: { layout: 'fullscreen' },
  title: 'App/Top bar',
} satisfies Meta<typeof TopBar>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
