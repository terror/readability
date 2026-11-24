import { ReactNode } from 'react';

export const TopBar = ({ actions }: { actions: ReactNode }) => (
  <header className='flex h-14 shrink-0 items-center justify-between gap-4 px-4 md:px-5'>
    <span className='text-[15px] font-semibold tracking-tight'>unclutter</span>
    <nav aria-label='Tools' className='flex items-center gap-1'>
      {actions}
    </nav>
  </header>
);
