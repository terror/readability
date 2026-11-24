import { Combobox } from '@/ui/combobox';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { article } from '../stories/fixtures';
import { SourcePanel } from './source-panel';

const Panel = () => {
  const [html, setHtml] = useState(
    '<article>\n  <h1>Riverside tram line approved</h1>\n  <p>After nearly eleven hours of public comment…</p>\n</article>'
  );

  return (
    <div className='h-[480px] w-[560px]'>
      <SourcePanel
        dark={document.documentElement.dataset.theme === 'dark'}
        html={html}
        onChange={setHtml}
        output={article.content}
        picker={
          <Combobox
            className='w-48'
            empty='No test page found.'
            label='Test page'
            onChange={() => {}}
            options={['ars-1', 'nytimes-1', 'wikipedia']}
            placeholder='Test pages'
            search='Search test pages'
            value='wikipedia'
          />
        }
      />
    </div>
  );
};

const meta = {
  component: Panel,
  title: 'App/Source panel',
} satisfies Meta<typeof Panel>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
