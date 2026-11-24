import { describe, expect, it } from 'bun:test';
import { readFile, readdir } from 'node:fs/promises';

import { defaultParserOptions, toOptions } from './parser-options';
import init, { isReadable, parse } from './unclutter-wasm/unclutter_wasm';

await init({
  module_or_path: await readFile(
    new URL('./unclutter-wasm/unclutter_wasm_bg.wasm', import.meta.url)
  ),
});

const samples = new URL('../../samples/', import.meta.url);

describe('parse', () => {
  it('extracts an article', () => {
    expect(
      parse(
        '<html><head><title>foo</title></head><body><p>bar</p></body></html>',
        undefined,
        undefined
      )
    ).toEqual({
      byline: undefined,
      content: '<div id="readability-page-1" class="page"><p>bar</p></div>',
      dir: undefined,
      excerpt: 'bar',
      lang: undefined,
      length: 3,
      publishedTime: undefined,
      siteName: undefined,
      textContent: 'bar',
      title: 'foo',
    });
  });

  it('resolves relative links against the base url', () => {
    expect(
      parse(
        '<p>foo <a href="baz">qux</a></p>',
        'https://foo.com/bar/',
        toOptions(defaultParserOptions)
      ).content
    ).toBe(
      '<div id="readability-page-1" class="page"><p>foo <a href="https://foo.com/bar/baz">qux</a></p></div>'
    );
  });

  it('parses every sample', async () => {
    const options = toOptions(defaultParserOptions);

    for (const name of await readdir(samples)) {
      const html = await readFile(new URL(name, samples), 'utf8');

      expect(isReadable(html)).toBe(true);
      expect(parse(html, undefined, options).length).toBeGreaterThan(0);
    }
  });
});
