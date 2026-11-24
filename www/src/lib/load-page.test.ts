import { describe, expect, it } from 'bun:test';

import { loadPage } from './load-page';

describe('loadPage', () => {
  it('requests the page through the proxy', async () => {
    let requested = '';

    expect(
      await loadPage('https://foo.com/?bar=baz', async (input) => {
        requested = input;
        return Response.json({ html: '<p>qux</p>', url: 'https://foo.com/' });
      })
    ).toEqual({ html: '<p>qux</p>', url: 'https://foo.com/' });

    expect(requested).toBe(
      '/api/fetch?url=https%3A%2F%2Ffoo.com%2F%3Fbar%3Dbaz'
    );
  });

  it('reports proxy errors', () => {
    for (const [response, message] of [
      [Response.json({ error: 'foo' }, { status: 502 }), 'foo'],
      [
        new Response('bar', { status: 500 }),
        'The playground server returned 500.',
      ],
    ] as const) {
      expect(loadPage('https://foo.com', async () => response)).rejects.toThrow(
        message
      );
    }
  });
});
