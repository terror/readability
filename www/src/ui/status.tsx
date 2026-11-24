import { ReactNode } from 'react';

const tones = {
  danger: 'text-danger',
  ok: 'text-ok',
  warn: 'text-warn',
};

export type StatusTone = keyof typeof tones;

export const Status = ({
  children,
  tone,
}: {
  children: ReactNode;
  tone: StatusTone;
}) => (
  <span
    className={`inline-flex items-center gap-1.5 text-[13px] ${tones[tone]}`}
  >
    <span aria-hidden='true' className='size-1.5 rounded-full bg-current' />
    {children}
  </span>
);
