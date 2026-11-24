import { ReactNode } from 'react';

interface TopBarProps {
  actions: ReactNode;
  center: ReactNode;
}

export const TopBar = ({ actions, center }: TopBarProps) => (
  <header className='grid shrink-0 grid-cols-[1fr_auto] items-center gap-x-4 gap-y-3 px-4 py-3 md:h-16 md:grid-cols-[1fr_auto_1fr] md:px-5 md:py-0'>
    <span className='text-[15px] font-semibold tracking-tight'>unclutter</span>
    <div className='col-span-2 row-start-2 flex justify-center md:col-span-1 md:row-start-auto'>
      {center}
    </div>
    <nav
      aria-label='Tools'
      className='flex items-center gap-1 justify-self-end'
    >
      {actions}
    </nav>
  </header>
);
