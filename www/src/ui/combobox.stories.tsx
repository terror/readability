import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { Combobox } from './combobox';

const pages = [
  'ars-1',
  'bbc-1',
  'nytimes-1',
  'nytimes-2',
  'rtl-1',
  'wikipedia',
];

const TestPages = ({ initial = 'wikipedia' }: { initial?: string }) => {
  const [value, setValue] = useState(initial);

  return (
    <Combobox
      className='w-44'
      empty='No test page found.'
      label='Test page'
      onChange={setValue}
      options={pages}
      placeholder='Test pages'
      search='Search test pages'
      value={value}
    />
  );
};

const meta = {
  component: TestPages,
  title: 'UI/Combobox',
} satisfies Meta<typeof TestPages>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Selected: Story = {};

export const Empty: Story = { args: { initial: '' } };
