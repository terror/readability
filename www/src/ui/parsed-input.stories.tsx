import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { ParsedInput } from './parsed-input';

const Count = () => {
  const [value, setValue] = useState(500);

  return (
    <ParsedInput
      aria-label='Min text length'
      inputMode='numeric'
      value={value}
      format={(value) => value.toString()}
      parse={(text) =>
        /^\d+$/.test(text) ? { value: Number(text) } : undefined
      }
      onValueChange={setValue}
    />
  );
};

const meta = {
  component: Count,
  title: 'UI/Parsed input',
} satisfies Meta<typeof Count>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
