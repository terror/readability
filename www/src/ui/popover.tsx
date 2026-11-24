import { ReactNode, useEffect, useId, useRef, useState } from 'react';

import { IconButton } from './icon-button';

interface PopoverProps {
  children: ReactNode;
  icon: ReactNode;
  label: string;
}

export const Popover = ({ children, icon, label }: PopoverProps) => {
  const id = useId();
  const ref = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const element = ref.current;

    if (!element) return;

    const onToggle = (event: Event) =>
      setOpen((event as ToggleEvent).newState === 'open');

    element.addEventListener('toggle', onToggle);

    return () => element.removeEventListener('toggle', onToggle);
  }, []);

  return (
    <>
      <IconButton
        label={label}
        active={open}
        aria-expanded={open}
        popoverTarget={id}
      >
        {icon}
      </IconButton>
      <div
        ref={ref}
        id={id}
        popover='auto'
        role='dialog'
        aria-label={label}
        className='bg-paper shadow-pop fixed inset-auto top-16 right-4 m-0 max-h-[calc(100dvh-5rem)] w-80 overflow-y-auto rounded-lg p-5 max-sm:inset-x-0 max-sm:top-auto max-sm:bottom-0 max-sm:max-h-[80dvh] max-sm:w-full max-sm:rounded-b-none'
      >
        {children}
      </div>
    </>
  );
};
