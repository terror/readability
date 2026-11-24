import { describe, expect, it } from 'bun:test';

import {
  defaultParserOptions,
  parseParserOptions,
  toOptions,
} from './parser-options';

describe('parseParserOptions', () => {
  it('falls back to defaults for invalid values', () => {
    expect(
      parseParserOptions({
        classesToPreserve: 'foo',
        keepClasses: true,
        linkDensityBias: 'bar',
        maxElements: -1,
        minTextLength: 1.5,
        nTopCandidates: 0,
      })
    ).toEqual({ ...defaultParserOptions, keepClasses: true });
  });

  it('falls back to defaults for non-objects', () => {
    expect(parseParserOptions('foo')).toEqual(defaultParserOptions);
  });
});

describe('toOptions', () => {
  it('omits a missing element limit', () => {
    expect(toOptions(defaultParserOptions)).toEqual({
      classesToPreserve: [],
      keepClasses: false,
      linkDensityBias: 0,
      maxElements: undefined,
      minTextLength: 500,
      nTopCandidates: 5,
      useJsonLd: true,
    });
  });

  it('passes the element limit', () => {
    expect(toOptions({ ...defaultParserOptions, maxElements: 1 })).toEqual({
      classesToPreserve: [],
      keepClasses: false,
      linkDensityBias: 0,
      maxElements: 1,
      minTextLength: 500,
      nTopCandidates: 5,
      useJsonLd: true,
    });
  });
});
