import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { LinkField } from './link-field';

const Field = ({
  initial = '',
  loading = false,
}: {
  initial?: string;
  loading?: boolean;
}) => {
  const [value, setValue] = useState(initial);

  return (
    <div className='w-[560px]'>
      <LinkField
        loading={loading}
        value={value}
        onChange={setValue}
        onSubmit={() => {}}
      />
    </div>
  );
};

const meta = {
  component: Field,
  title: 'UI/Link field',
} satisfies Meta<typeof Field>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Empty: Story = {};

export const Filled: Story = {
  args: { initial: 'harborledger.example/news/riverside-tram-line' },
};

export const Loading: Story = {
  args: {
    initial: 'harborledger.example/news/riverside-tram-line',
    loading: true,
  },
};
