import { ReactNode } from 'react';

export const Section = ({
  action,
  children,
  title,
}: {
  action?: ReactNode;
  children: ReactNode;
  title: string;
}) => (
  <section className='flex flex-col gap-2.5'>
    <div className='flex items-center justify-between'>
      <h2 className='text-ink-3 text-xs font-medium'>{title}</h2>
      {action}
    </div>
    {children}
  </section>
);

export const Row = ({
  children,
  label,
}: {
  children: ReactNode;
  label: string;
}) => (
  <label className='border-line flex min-h-11 items-center justify-between gap-4 border-b text-sm last:border-b-0'>
    <span>{label}</span>
    {children}
  </label>
);
