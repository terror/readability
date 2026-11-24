import { ParserOptions, defaultParserOptions } from '@/lib/parser-options';
import type { ParseResult } from '@/lib/types';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { result } from '../stories/fixtures';
import { DetailsPanel } from './details-panel';

const Panel = ({ result }: { result?: ParseResult }) => {
  const [options, setOptions] = useState<ParserOptions>(defaultParserOptions);

  return (
    <div className='w-72'>
      <DetailsPanel
        options={options}
        result={result}
        updateOptions={(update) =>
          setOptions((previous) => ({ ...previous, ...update }))
        }
      />
    </div>
  );
};

const meta = {
  component: Panel,
  title: 'App/Details panel',
} satisfies Meta<typeof Panel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Readable: Story = { args: { result } };

export const Failed: Story = {
  args: {
    result: {
      duration: 0.4,
      message: 'missing article content',
      readable: false,
      status: 'error',
    },
  },
};
