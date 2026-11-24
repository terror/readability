import { Appearance, TEXT_SIZES, THEMES, Theme } from '@/lib/appearance';
import { Section } from '@/ui/section';
import { SegmentedControl } from '@/ui/segmented-control';

interface AppearancePanelProps {
  appearance: Appearance;
  updateAppearance: (appearance: Partial<Appearance>) => void;
}

const themeNames: Record<Theme, string> = {
  dark: 'Dark',
  light: 'Light',
  sepia: 'Sepia',
};

export const AppearancePanel = ({
  appearance,
  updateAppearance,
}: AppearancePanelProps) => {
  const sizeIndex = TEXT_SIZES.findIndex(
    (size) => size === appearance.textSize
  );

  return (
    <div className='flex flex-col gap-5'>
      <Section title='Theme'>
        <div
          role='radiogroup'
          aria-label='Theme'
          className='flex justify-around'
        >
          {THEMES.map((theme) => (
            <button
              key={theme}
              type='button'
              role='radio'
              aria-checked={appearance.theme === theme}
              onClick={() => updateAppearance({ theme })}
              className='group text-ink-2 aria-checked:text-ink flex cursor-pointer flex-col items-center gap-1.5 text-xs'
            >
              <span
                data-theme={theme}
                className='bg-paper text-ink font-read flex size-11 items-center justify-center rounded-full text-[15px] font-medium shadow-[0_0_0_1px_var(--line)] group-aria-checked:shadow-[0_0_0_1px_var(--line),0_0_0_3px_var(--paper),0_0_0_5px_var(--accent)]'
              >
                Aa
              </span>
              {themeNames[theme]}
            </button>
          ))}
        </div>
      </Section>
      <Section title='Typeface'>
        <SegmentedControl
          label='Typeface'
          value={appearance.typeface}
          onChange={(typeface) => updateAppearance({ typeface })}
          options={[
            { label: 'Serif', value: 'serif' },
            { label: 'Sans', value: 'sans' },
          ]}
        />
      </Section>
      <Section title='Text size'>
        <div className='bg-wash flex h-9 items-center justify-between rounded-md px-1'>
          <button
            type='button'
            aria-label='Smaller text'
            disabled={sizeIndex <= 0}
            onClick={() =>
              updateAppearance({ textSize: TEXT_SIZES[sizeIndex - 1] })
            }
            className='text-ink-2 hover:text-ink font-read h-7 w-9 cursor-pointer rounded-sm text-[13px] disabled:cursor-default disabled:opacity-40'
          >
            A
          </button>
          <span className='text-[13px] font-medium tabular-nums'>
            {appearance.textSize}
          </span>
          <button
            type='button'
            aria-label='Larger text'
            disabled={sizeIndex >= TEXT_SIZES.length - 1}
            onClick={() =>
              updateAppearance({ textSize: TEXT_SIZES[sizeIndex + 1] })
            }
            className='text-ink-2 hover:text-ink font-read h-7 w-9 cursor-pointer rounded-sm text-[19px] disabled:cursor-default disabled:opacity-40'
          >
            A
          </button>
        </div>
      </Section>
      <Section title='Line width'>
        <SegmentedControl
          label='Line width'
          value={appearance.width}
          onChange={(width) => updateAppearance({ width })}
          options={[
            { label: 'Narrow', value: 'narrow' },
            { label: 'Wide', value: 'wide' },
          ]}
        />
      </Section>
    </div>
  );
};
