import { describe, expect, it } from 'bun:test';

import { defaultAppearance, parseAppearance } from './appearance';

describe('parseAppearance', () => {
  it('keeps valid values and falls back for invalid ones', () => {
    expect(
      parseAppearance({
        textSize: 17,
        theme: 'sepia',
        typeface: 'foo',
        width: 'wide',
      })
    ).toEqual({ ...defaultAppearance, theme: 'sepia', width: 'wide' });
  });

  it('uses the given fallback', () => {
    expect(
      parseAppearance('foo', { ...defaultAppearance, theme: 'dark' })
    ).toEqual({ ...defaultAppearance, theme: 'dark' });
  });
});
