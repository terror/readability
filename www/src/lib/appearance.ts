import { z } from 'zod';

export const THEMES = ['light', 'sepia', 'dark'] as const;
export const TYPEFACES = ['serif', 'sans'] as const;
export const WIDTHS = ['narrow', 'wide'] as const;
export const TEXT_SIZES = [16, 18, 20, 22, 24, 26] as const;

export type TextSize = (typeof TEXT_SIZES)[number];
export type Theme = (typeof THEMES)[number];

export interface Appearance {
  textSize: TextSize;
  theme: Theme;
  typeface: (typeof TYPEFACES)[number];
  width: (typeof WIDTHS)[number];
}

export const defaultAppearance: Appearance = {
  textSize: 20,
  theme: 'light',
  typeface: 'serif',
  width: 'narrow',
};

export function parseAppearance(
  value: unknown,
  fallback: Appearance = defaultAppearance
): Appearance {
  return z
    .object({
      textSize: z.literal(TEXT_SIZES).catch(fallback.textSize),
      theme: z.enum(THEMES).catch(fallback.theme),
      typeface: z.enum(TYPEFACES).catch(fallback.typeface),
      width: z.enum(WIDTHS).catch(fallback.width),
    })
    .catch(fallback)
    .parse(value);
}
