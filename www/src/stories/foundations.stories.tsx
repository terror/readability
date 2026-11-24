import { THEMES } from '@/lib/appearance';
import type { Meta, StoryObj } from '@storybook/react-vite';

const tokens = [
  'paper',
  'ink',
  'ink-2',
  'ink-3',
  'line',
  'wash',
  'accent',
  'accent-wash',
  'ok',
  'warn',
  'danger',
];

const Colors = () => (
  <div className='grid grid-cols-3 gap-4'>
    {THEMES.map((theme) => (
      <div
        key={theme}
        data-theme={theme}
        className='bg-paper text-ink flex flex-col gap-3 rounded-lg p-4 shadow-[0_0_0_1px_var(--line)]'
      >
        <span className='text-sm font-semibold capitalize'>{theme}</span>
        {tokens.map((token) => (
          <div key={token} className='flex items-center gap-2.5 text-xs'>
            <span
              className='size-6 rounded-sm shadow-[0_0_0_1px_var(--line)]'
              style={{ background: `var(--${token})` }}
            />
            <span className='font-code'>--{token}</span>
          </div>
        ))}
      </div>
    ))}
  </div>
);

const specimens = [
  [
    'Literata 500 · title',
    'font-read text-[40px] leading-[1.15] font-medium tracking-[-0.015em]',
    'Riverside tram line',
  ],
  [
    'Literata 600 · heading',
    'font-read text-2xl leading-[1.3] font-semibold',
    'What the plan includes',
  ],
  [
    'Literata 400 · body',
    'font-read text-xl leading-[1.7]',
    'Trams will run every six minutes.',
  ],
  ['Inter 500 · interface', 'text-[15px] font-medium', 'Source · Details'],
  [
    'Inter 400 · meta',
    'text-ink-2 text-sm',
    'Mira Castellanos · March 14, 2026 · 4 min',
  ],
  [
    'JetBrains Mono · code',
    'font-code text-ink-2 text-[12.5px]',
    '<article class="story">',
  ],
] as const;

const Typography = () => (
  <div className='flex w-[720px] flex-col'>
    {specimens.map(([label, className, sample]) => (
      <div
        key={label}
        className='border-line grid grid-cols-[180px_1fr] items-baseline gap-4 border-b py-3'
      >
        <span className='font-code text-ink-3 text-xs'>{label}</span>
        <span className={className}>{sample}</span>
      </div>
    ))}
  </div>
);

const meta = {
  title: 'Foundations',
} satisfies Meta;

export default meta;

export const Color: StoryObj = { render: () => <Colors /> };

export const Type: StoryObj = { render: () => <Typography /> };
