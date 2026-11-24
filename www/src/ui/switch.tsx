import { ComponentProps } from 'react';

export const Switch = ({
  className = '',
  ...props
}: Omit<ComponentProps<'input'>, 'type'>) => (
  <span className={`relative inline-flex shrink-0 ${className}`}>
    <input
      type='checkbox'
      role='switch'
      className='peer absolute inset-0 cursor-pointer opacity-0 disabled:cursor-default'
      {...props}
    />
    <span
      aria-hidden='true'
      className='bg-line peer-checked:bg-ink peer-focus-visible:outline-accent pointer-events-none h-[22px] w-9 rounded-full transition-colors peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-disabled:opacity-40 peer-checked:[&>span]:translate-x-[14px]'
    >
      <span className='bg-paper absolute top-[3px] left-[3px] size-4 rounded-full transition-transform' />
    </span>
  </span>
);
