import { ReactNode } from 'react';

const tones = {
  danger: 'text-danger',
  warn: 'text-warn',
};

interface NoticeProps {
  actions?: ReactNode;
  children: ReactNode;
  title: string;
  tone: keyof typeof tones;
}

export const Notice = ({ actions, children, title, tone }: NoticeProps) => (
  <div
    role={tone === 'danger' ? 'alert' : 'status'}
    className='flex flex-col gap-3 text-sm leading-relaxed'
  >
    <p className='text-ink-2'>
      <span className={`font-medium ${tones[tone]}`}>{title}</span> {children}
    </p>
    {actions && <div className='flex flex-wrap gap-2'>{actions}</div>}
  </div>
);
