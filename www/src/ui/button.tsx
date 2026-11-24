import { ComponentProps } from 'react';

const variants = {
  primary: 'bg-ink text-paper hover:opacity-85',
  secondary: 'bg-wash text-ink hover:bg-line',
  quiet: 'text-ink-2 hover:text-ink shadow-[inset_0_0_0_1px_var(--line)]',
};

export type ButtonVariant = keyof typeof variants;

export const Button = ({
  className = '',
  variant = 'secondary',
  ...props
}: ComponentProps<'button'> & { variant?: ButtonVariant }) => (
  <button
    type='button'
    className={`inline-flex h-9 cursor-pointer items-center justify-center gap-2 rounded-md px-3.5 text-sm font-medium whitespace-nowrap transition-[opacity,background-color,color] disabled:cursor-default disabled:opacity-40 ${variants[variant]} ${className}`}
    {...props}
  />
);
