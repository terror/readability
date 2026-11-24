import { z } from 'zod';

import type { Options } from './types';

export interface ParserOptions {
  classesToPreserve: string[];
  keepClasses: boolean;
  linkDensityBias: number;
  maxElements: number | null;
  minTextLength: number;
  nTopCandidates: number;
  useJsonLd: boolean;
}

export const defaultParserOptions: ParserOptions = {
  classesToPreserve: [],
  keepClasses: false,
  linkDensityBias: 0,
  maxElements: null,
  minTextLength: 500,
  nTopCandidates: 5,
  useJsonLd: true,
};

const count = z.number().int().nonnegative();

const parserOptionsSchema = z
  .object({
    classesToPreserve: z
      .array(z.string())
      .catch(defaultParserOptions.classesToPreserve),
    keepClasses: z.boolean().catch(defaultParserOptions.keepClasses),
    linkDensityBias: z
      .number()
      .finite()
      .catch(defaultParserOptions.linkDensityBias),
    maxElements: count.nullable().catch(defaultParserOptions.maxElements),
    minTextLength: count.catch(defaultParserOptions.minTextLength),
    nTopCandidates: count.positive().catch(defaultParserOptions.nTopCandidates),
    useJsonLd: z.boolean().catch(defaultParserOptions.useJsonLd),
  })
  .catch(defaultParserOptions);

export function parseParserOptions(value: unknown): ParserOptions {
  return parserOptionsSchema.parse(value);
}

export function toOptions({ maxElements, ...options }: ParserOptions): Options {
  return { ...options, maxElements: maxElements ?? undefined };
}
