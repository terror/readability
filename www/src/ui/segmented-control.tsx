interface SegmentedControlProps<T extends string> {
  label: string;
  onChange: (value: T) => void;
  options: readonly { label: string; value: T }[];
  value: T;
}

export const SegmentedControl = <T extends string>({
  label,
  onChange,
  options,
  value,
}: SegmentedControlProps<T>) => (
  <div
    role='radiogroup'
    aria-label={label}
    className='bg-wash grid auto-cols-fr grid-flow-col gap-0.5 rounded-md p-[3px]'
  >
    {options.map((option) => (
      <button
        key={option.value}
        type='button'
        role='radio'
        aria-checked={option.value === value}
        onClick={() => onChange(option.value)}
        className='text-ink-2 hover:text-ink aria-checked:bg-paper aria-checked:text-ink h-[30px] cursor-pointer rounded-sm px-3 text-[13px] whitespace-nowrap aria-checked:font-medium aria-checked:shadow-[0_1px_2px_rgb(0_0_0/0.12)]'
      >
        {option.label}
      </button>
    ))}
  </div>
);
