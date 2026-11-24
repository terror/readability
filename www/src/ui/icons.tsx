import { ReactNode, SVGProps } from 'react';

const Icon = ({
  children,
  size = 18,
  ...props
}: SVGProps<SVGSVGElement> & { children: ReactNode; size?: number }) => (
  <svg
    width={size}
    height={size}
    viewBox='0 0 24 24'
    fill='none'
    stroke='currentColor'
    strokeWidth={1.75}
    strokeLinecap='round'
    strokeLinejoin='round'
    aria-hidden='true'
    {...props}
  >
    {children}
  </svg>
);

type IconProps = SVGProps<SVGSVGElement> & { size?: number };

export const ArrowIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='M5 12h14' />
    <path d='m13 6 6 6-6 6' />
  </Icon>
);

export const ChevronIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='m6 9 6 6 6-6' />
  </Icon>
);

export const CodeIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='m8 8-4 4 4 4' />
    <path d='m16 8 4 4-4 4' />
  </Icon>
);

export const InfoIcon = (props: IconProps) => (
  <Icon {...props}>
    <circle cx='12' cy='12' r='9' />
    <path d='M12 11v5' />
    <path d='M12 8h.01' />
  </Icon>
);

export const LinkIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1' />
    <path d='M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1' />
  </Icon>
);

export const SpinnerIcon = (props: IconProps) => (
  <Icon {...props} className='animate-spin'>
    <path d='M12 3a9 9 0 1 0 9 9' />
  </Icon>
);
