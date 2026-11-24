import {
  ParserOptions,
  defaultParserOptions,
  parseParserOptions,
} from '@/lib/parser-options';
import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useEffect, useState } from 'react';

const OPTIONS_STORAGE_KEY = 'unclutter:parser-options';

export function useParserOptions() {
  const [options, setOptions] = useState<ParserOptions>(() => {
    const saved = readStorage(OPTIONS_STORAGE_KEY);

    if (saved === null) return defaultParserOptions;

    try {
      return parseParserOptions(JSON.parse(saved));
    } catch (error) {
      console.warn(
        `Error reading ${OPTIONS_STORAGE_KEY} from localStorage:`,
        error
      );

      return defaultParserOptions;
    }
  });

  useEffect(() => {
    writeStorage(OPTIONS_STORAGE_KEY, JSON.stringify(options));
  }, [options]);

  const updateOptions = useCallback((options: Partial<ParserOptions>) => {
    setOptions((previous) => ({ ...previous, ...options }));
  }, []);

  return { options, updateOptions };
}
