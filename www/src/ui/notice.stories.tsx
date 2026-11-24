import type { Meta, StoryObj } from '@storybook/react-vite';

import { Button } from './button';
import { Notice } from './notice';

const meta = {
  args: {
    children:
      'Some sites block automated requests; you can save the page and paste its HTML instead.',
    title: 'Couldn’t load harborledger.example.',
    tone: 'danger',
  },
  component: Notice,
  decorators: [
    (Story) => (
      <div className='w-[560px]'>
        <Story />
      </div>
    ),
  ],
  title: 'UI/Notice',
} satisfies Meta<typeof Notice>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Danger: Story = {
  args: {
    actions: (
      <>
        <Button variant='primary'>Try again</Button>
        <Button>Paste HTML</Button>
      </>
    ),
  },
};

export const Warn: Story = {
  args: {
    children:
      'The result can be incomplete. Lowering the minimum text length in Details can help.',
    title: 'This page may not be an article.',
    tone: 'warn',
  },
};
