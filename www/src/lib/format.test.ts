import { describe, expect, it } from 'bun:test';

import { formatDate, readingTime } from './format';

describe('formatDate', () => {
  it('formats dates and keeps anything else', () => {
    expect(formatDate('2026-03-14T23:30:00Z')).toBe('March 14, 2026');
    expect(formatDate('foo')).toBe('foo');
  });
});

describe('readingTime', () => {
  it('rounds to whole minutes with a minimum of one', () => {
    expect(readingTime('foo bar')).toBe('1 min');
    expect(readingTime('foo '.repeat(1000))).toBe('4 min');
  });
});
