import { ComponentProps } from 'react';

export const IconButton = ({
  active = false,
  className = '',
  label,
  ...props
}: ComponentProps<'button'> & { active?: boolean; label: string }) => (
  <button
    type='button'
    aria-label={label}
    title={label}
    className={`inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-md transition-colors ${active ? 'bg-wash text-ink' : 'text-ink-2 hover:bg-wash hover:text-ink'} ${className}`}
    {...props}
  />
);
