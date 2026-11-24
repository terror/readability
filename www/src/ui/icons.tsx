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

export const CheckIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='m5 12 5 5L20 7' />
  </Icon>
);

export const ChevronsUpDownIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='m7 15 5 5 5-5' />
    <path d='m7 9 5-5 5 5' />
  </Icon>
);

export const CodeIcon = (props: IconProps) => (
  <Icon {...props}>
    <path d='m8 8-4 4 4 4' />
    <path d='m16 8 4 4-4 4' />
  </Icon>
);

export const GripVerticalIcon = (props: IconProps) => (
  <Icon {...props}>
    <circle cx='9' cy='6' r='1' />
    <circle cx='9' cy='12' r='1' />
    <circle cx='9' cy='18' r='1' />
    <circle cx='15' cy='6' r='1' />
    <circle cx='15' cy='12' r='1' />
    <circle cx='15' cy='18' r='1' />
  </Icon>
);

export const InfoIcon = (props: IconProps) => (
  <Icon {...props}>
    <circle cx='12' cy='12' r='9' />
    <path d='M12 11v5' />
    <path d='M12 8h.01' />
  </Icon>
);

export const SearchIcon = (props: IconProps) => (
  <Icon {...props}>
    <circle cx='11' cy='11' r='7' />
    <path d='m20 20-3.5-3.5' />
  </Icon>
);

export const SpinnerIcon = (props: IconProps) => (
  <Icon {...props} className='animate-spin'>
    <path d='M12 3a9 9 0 1 0 9 9' />
  </Icon>
);
