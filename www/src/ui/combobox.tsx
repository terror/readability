import { cn } from '@/lib/utils';
import * as PopoverPrimitive from '@radix-ui/react-popover';
import { useState } from 'react';

import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from './command';
import { CheckIcon, ChevronsUpDownIcon } from './icons';

interface ComboboxProps {
  align?: 'center' | 'end' | 'start';
  className?: string;
  empty: string;
  label: string;
  onChange: (value: string) => void;
  options: readonly string[];
  placeholder: string;
  search: string;
  value: string;
}

export const Combobox = ({
  align = 'start',
  className,
  empty,
  label,
  onChange,
  options,
  placeholder,
  search,
  value,
}: ComboboxProps) => {
  const [open, setOpen] = useState(false);

  return (
    <PopoverPrimitive.Root open={open} onOpenChange={setOpen}>
      <PopoverPrimitive.Trigger asChild>
        <button
          type='button'
          role='combobox'
          aria-expanded={open}
          aria-label={label}
          className={cn(
            'bg-wash hover:bg-line data-[state=open]:bg-line flex h-10 min-w-0 cursor-pointer items-center justify-between gap-2 rounded-full pr-3 pl-4 text-sm transition-colors outline-none focus-visible:shadow-[0_0_0_2px_var(--accent)]',
            className
          )}
        >
          <span className={cn('truncate', value ? 'text-ink' : 'text-ink-3')}>
            {value || placeholder}
          </span>
          <ChevronsUpDownIcon size={15} className='text-ink-3 shrink-0' />
        </button>
      </PopoverPrimitive.Trigger>
      <PopoverPrimitive.Portal>
        <PopoverPrimitive.Content
          align={align}
          sideOffset={8}
          collisionPadding={16}
          className='shadow-pop data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-64 origin-(--radix-popover-content-transform-origin) rounded-lg outline-none'
        >
          <Command
            filter={(item, query) =>
              item.toLowerCase().includes(query.trim().toLowerCase()) ? 1 : 0
            }
          >
            <CommandInput placeholder={search} />
            <CommandList>
              <CommandEmpty>{empty}</CommandEmpty>
              <CommandGroup>
                {options.map((option) => (
                  <CommandItem
                    key={option}
                    value={option}
                    onSelect={() => {
                      setOpen(false);
                      if (option !== value) onChange(option);
                    }}
                  >
                    <span className='font-code min-w-0 flex-1 truncate text-[13px]'>
                      {option}
                    </span>
                    {option === value && (
                      <CheckIcon size={15} className='text-accent shrink-0' />
                    )}
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverPrimitive.Content>
      </PopoverPrimitive.Portal>
    </PopoverPrimitive.Root>
  );
};
