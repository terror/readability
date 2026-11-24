import { cn } from '@/lib/utils';
import { Command as CommandPrimitive } from 'cmdk';
import { ComponentProps } from 'react';

import { SearchIcon } from './icons';

function Command({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive>) {
  return (
    <CommandPrimitive
      data-slot='command'
      className={cn(
        'bg-paper text-ink flex h-full w-full flex-col overflow-hidden rounded-lg',
        className
      )}
      {...props}
    />
  );
}

function CommandInput({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive.Input>) {
  return (
    <div
      data-slot='command-input-wrapper'
      className='border-line text-ink-3 flex h-11 items-center gap-2.5 border-b px-3.5'
    >
      <SearchIcon size={16} />
      <CommandPrimitive.Input
        data-slot='command-input'
        className={cn(
          'text-ink placeholder:text-ink-3 flex h-full w-full bg-transparent text-sm outline-none disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        {...props}
      />
    </div>
  );
}

function CommandList({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive.List>) {
  return (
    <CommandPrimitive.List
      data-slot='command-list'
      className={cn(
        'max-h-80 scroll-py-1.5 overflow-x-hidden overflow-y-auto overscroll-contain',
        className
      )}
      {...props}
    />
  );
}

function CommandEmpty({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive.Empty>) {
  return (
    <CommandPrimitive.Empty
      data-slot='command-empty'
      className={cn('text-ink-3 py-8 text-center text-sm', className)}
      {...props}
    />
  );
}

function CommandGroup({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive.Group>) {
  return (
    <CommandPrimitive.Group
      data-slot='command-group'
      className={cn(
        'text-ink [&_[cmdk-group-heading]]:text-ink-3 overflow-hidden p-1.5 [&_[cmdk-group-heading]]:px-2.5 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium',
        className
      )}
      {...props}
    />
  );
}

function CommandItem({
  className,
  ...props
}: ComponentProps<typeof CommandPrimitive.Item>) {
  return (
    <CommandPrimitive.Item
      data-slot='command-item'
      className={cn(
        'text-ink-2 data-[selected=true]:bg-wash data-[selected=true]:text-ink relative flex cursor-pointer items-center gap-2 rounded-md px-2.5 py-2 text-sm outline-none select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50',
        className
      )}
      {...props}
    />
  );
}

export {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
};
