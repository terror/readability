import { FormEvent, useId } from 'react';

import { ArrowIcon, LinkIcon, SpinnerIcon } from './icons';

interface LinkFieldProps {
  loading?: boolean;
  onChange: (value: string) => void;
  onSubmit: (value: string) => void;
  value: string;
}

export const LinkField = ({
  loading = false,
  onChange,
  onSubmit,
  value,
}: LinkFieldProps) => {
  const id = useId();

  return (
    <form
      role='search'
      onSubmit={(event: FormEvent) => {
        event.preventDefault();
        if (value.trim()) onSubmit(value);
      }}
      className='bg-wash text-ink-3 flex h-10 w-full min-w-0 items-center gap-2.5 rounded-full pr-1 pl-3.5 transition-shadow focus-within:shadow-[0_0_0_2px_var(--accent)]'
    >
      <LinkIcon size={16} />
      <label htmlFor={id} className='sr-only'>
        Article URL
      </label>
      <input
        id={id}
        type='text'
        inputMode='url'
        autoComplete='url'
        spellCheck={false}
        placeholder='Paste a link to an article'
        value={value}
        onChange={(event) => onChange(event.target.value)}
        className='text-ink placeholder:text-ink-3 min-w-0 flex-1 bg-transparent text-sm outline-none'
      />
      <button
        type='submit'
        aria-label='Read'
        title='Read'
        disabled={loading}
        className='bg-ink text-paper inline-flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-full transition-opacity hover:opacity-85 disabled:cursor-default'
      >
        {loading ? <SpinnerIcon size={16} /> : <ArrowIcon size={16} />}
      </button>
    </form>
  );
};
