import { ComponentProps, useState } from 'react';

export interface ParsedInputProps<T> extends Omit<
  ComponentProps<'input'>,
  'onChange' | 'value'
> {
  format: (value: T) => string;
  onValueChange: (value: T) => void;
  parse: (text: string) => { value: T } | undefined;
  value: T;
}

export const ParsedInput = <T,>({
  className = '',
  format,
  onValueChange,
  parse,
  value,
  ...props
}: ParsedInputProps<T>) => {
  const [draft, setDraft] = useState(() => format(value));
  const [previous, setPrevious] = useState(value);

  if (format(value) !== format(previous)) {
    setPrevious(value);

    const parsed = parse(draft);

    if (!parsed || format(parsed.value) !== format(value)) {
      setDraft(format(value));
    }
  }

  return (
    <input
      {...props}
      aria-invalid={parse(draft) === undefined}
      value={draft}
      onChange={(event) => {
        setDraft(event.target.value);

        const parsed = parse(event.target.value);

        if (parsed) onValueChange(parsed.value);
      }}
      className={`bg-wash text-ink placeholder:text-ink-3 font-code h-8 w-24 rounded-md px-2.5 text-right text-[13px] outline-none disabled:opacity-40 aria-invalid:shadow-[inset_0_0_0_1px_var(--danger)] ${className}`}
    />
  );
};
