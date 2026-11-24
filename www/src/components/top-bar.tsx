import { ReactNode } from 'react';

interface TopBarProps {
  actions: ReactNode;
  field: ReactNode;
}

export const TopBar = ({ actions, field }: TopBarProps) => (
  <header className='grid shrink-0 grid-cols-[1fr_auto] items-center gap-x-4 gap-y-3 px-4 py-3 md:h-16 md:grid-cols-[1fr_minmax(0,35rem)_1fr] md:px-5 md:py-0'>
    <span className='text-[15px] font-semibold tracking-tight'>unclutter</span>
    <div className='col-span-2 row-start-2 md:col-span-1 md:row-start-auto'>
      {field}
    </div>
    <nav
      aria-label='Tools'
      className='flex items-center gap-1 justify-self-end'
    >
      {actions}
    </nav>
  </header>
);
