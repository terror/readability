import { samples } from '@/lib/samples';
import { SegmentedControl } from '@/ui/segmented-control';
import { useState } from 'react';

import { CodeView } from './code-view';

interface SourcePanelProps {
  dark: boolean;
  html: string;
  onChange: (html: string) => void;
  onOpenSample: (name: string) => void;
  output?: string;
  sample?: string;
}

export const SourcePanel = ({
  dark,
  html,
  onChange,
  onOpenSample,
  output,
  sample,
}: SourcePanelProps) => {
  const [view, setView] = useState<'input' | 'output'>('input');

  return (
    <section aria-label='Source' className='flex h-full min-h-0 flex-col'>
      <div className='flex h-12 shrink-0 items-center gap-4 px-5 text-[13px]'>
        <SegmentedControl
          label='Source view'
          value={view}
          onChange={setView}
          options={[
            { label: 'Input', value: 'input' },
            { label: 'Output', value: 'output' },
          ]}
        />
        <div className='text-ink-2 ml-auto flex gap-3.5'>
          {samples.map(({ name }) => (
            <button
              key={name}
              type='button'
              aria-pressed={name === sample}
              onClick={() => {
                setView('input');
                onOpenSample(name);
              }}
              className='hover:text-ink aria-pressed:text-ink decoration-accent cursor-pointer capitalize underline-offset-4 aria-pressed:underline'
            >
              {name}
            </button>
          ))}
        </div>
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
