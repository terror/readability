import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { SegmentedControl } from './segmented-control';

const Control = () => {
  const [value, setValue] = useState<'serif' | 'sans'>('serif');

  return (
    <div className='w-60'>
      <SegmentedControl
        label='Typeface'
        value={value}
        onChange={setValue}
        options={[
          { label: 'Serif', value: 'serif' },
          { label: 'Sans', value: 'sans' },
        ]}
      />
    </div>
  );
};

const meta = {
  component: Control,
  title: 'UI/Segmented control',
} satisfies Meta<typeof Control>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
