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
