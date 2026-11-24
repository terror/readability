import { Combobox } from '@/ui/combobox';
import { IconButton } from '@/ui/icon-button';
import { CodeIcon, InfoIcon } from '@/ui/icons';
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
    center: (
      <Combobox
        className='w-full md:w-56'
        empty='No test page found.'
        label='Test page'
        onChange={() => {}}
        options={['ars-1', 'nytimes-1', 'wikipedia']}
        placeholder='Test pages'
        search='Search test pages'
        value='wikipedia'
      />
    ),
  },
  component: TopBar,
  parameters: { layout: 'fullscreen' },
  title: 'App/Top bar',
} satisfies Meta<typeof TopBar>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
