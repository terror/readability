import {
  Appearance,
  defaultAppearance,
  parseAppearance,
} from '@/lib/appearance';
import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useLayoutEffect, useState } from 'react';

const APPEARANCE_STORAGE_KEY = 'unclutter:appearance';

export function useAppearance() {
  const [appearance, setAppearance] = useState<Appearance>(() => {
    const fallback: Appearance = {
      ...defaultAppearance,
      theme: window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light',
    };

    const saved = readStorage(APPEARANCE_STORAGE_KEY);

    if (saved === null) return fallback;

    try {
      return parseAppearance(JSON.parse(saved), fallback);
    } catch {
      return fallback;
    }
  });

  useLayoutEffect(() => {
    document.documentElement.dataset.theme = appearance.theme;
    writeStorage(APPEARANCE_STORAGE_KEY, JSON.stringify(appearance));
  }, [appearance]);

  const updateAppearance = useCallback((update: Partial<Appearance>) => {
    setAppearance((previous) => ({ ...previous, ...update }));
  }, []);

  return { appearance, updateAppearance };
}
