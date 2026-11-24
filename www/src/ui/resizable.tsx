import { cn } from '@/lib/utils';
import { ComponentProps } from 'react';
import * as ResizablePrimitive from 'react-resizable-panels';

import { GripVerticalIcon } from './icons';

function ResizablePanelGroup({
  className,
  ...props
}: ComponentProps<typeof ResizablePrimitive.Group>) {
  return (
    <ResizablePrimitive.Group
      data-slot='resizable-panel-group'
      className={cn('h-full w-full', className)}
      {...props}
    />
  );
}

function ResizablePanel({
  ...props
}: ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot='resizable-panel' {...props} />;
}

function ResizableHandle({
  className,
  withHandle,
  ...props
}: ComponentProps<typeof ResizablePrimitive.Separator> & {
  withHandle?: boolean;
}) {
  return (
    <ResizablePrimitive.Separator
      data-slot='resizable-handle'
      className={cn(
        'bg-line hover:bg-accent focus-visible:bg-accent relative flex w-px items-center justify-center transition-colors outline-none after:absolute after:inset-y-0 after:left-1/2 after:w-2 after:-translate-x-1/2 aria-[orientation=horizontal]:h-px aria-[orientation=horizontal]:w-full aria-[orientation=horizontal]:after:left-0 aria-[orientation=horizontal]:after:h-2 aria-[orientation=horizontal]:after:w-full aria-[orientation=horizontal]:after:translate-x-0 aria-[orientation=horizontal]:after:-translate-y-1/2 [&[aria-orientation=horizontal]>div]:rotate-90',
        className
      )}
      {...props}
    >
      {withHandle && (
        <div className='bg-paper text-ink-3 z-10 flex h-6 w-3 items-center justify-center rounded-sm shadow-[0_0_0_1px_var(--line)]'>
          <GripVerticalIcon size={10} />
        </div>
      )}
    </ResizablePrimitive.Separator>
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };
