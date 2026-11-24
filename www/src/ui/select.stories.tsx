import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { Select } from './select';

const Pages = () => {
  const [value, setValue] = useState('wikipedia');

  return (
    <Select
      aria-label='Test page'
      className='w-48'
      value={value}
      onChange={(event) => setValue(event.target.value)}
    >
      {['ars-1', 'nytimes-1', 'wikipedia'].map((name) => (
        <option key={name} value={name}>
          {name}
        </option>
      ))}
    </Select>
  );
};

const meta = {
  component: Pages,
  title: 'UI/Select',
} satisfies Meta<typeof Pages>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
