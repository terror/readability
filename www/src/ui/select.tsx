import { ComponentProps } from 'react';

import { ChevronIcon } from './icons';

export const Select = ({
  className = '',
  ...props
}: ComponentProps<'select'>) => (
  <span className={`relative inline-flex ${className}`}>
    <select
      className='bg-wash text-ink h-10 w-full min-w-0 cursor-pointer appearance-none truncate rounded-full pr-9 pl-4 text-sm outline-none focus-visible:shadow-[0_0_0_2px_var(--accent)]'
      {...props}
    />
    <ChevronIcon
      size={16}
      className='text-ink-3 pointer-events-none absolute top-1/2 right-3.5 -translate-y-1/2'
    />
  </span>
);
