import { SegmentedControl } from '@/ui/segmented-control';
import { useState } from 'react';

import { CodeView } from './code-view';

interface SourcePanelProps {
  dark: boolean;
  html: string;
  onChange: (html: string) => void;
  output?: string;
}

export const SourcePanel = ({
  dark,
  html,
  onChange,
  output,
}: SourcePanelProps) => {
  const [view, setView] = useState<'input' | 'output'>('input');

  return (
    <section aria-label='Source' className='flex h-full min-h-0 flex-col'>
      <div className='flex h-12 shrink-0 items-center px-5'>
        <SegmentedControl
          label='Source view'
          value={view}
          onChange={setView}
          options={[
            { label: 'Input', value: 'input' },
            { label: 'Output', value: 'output' },
          ]}
        />
      </div>
      <div className='min-h-0 flex-1'>
        {view === 'input' ? (
          <CodeView
            dark={dark}
            label='HTML input'
            onChange={onChange}
            value={html}
          />
        ) : (
          <CodeView
            dark={dark}
            label='Extracted HTML'
            readOnly
            value={output ?? ''}
          />
        )}
      </div>
    </section>
  );
};
