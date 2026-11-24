import { describe, expect, it } from 'bun:test';

import {
  type Dependencies,
  fetchPage,
  handle,
  isPrivateAddress,
} from './fetch-page';

const resolve = async () => ['93.184.215.14'];

const respond =
  (...responses: Response[]) =>
  async () =>
    responses.shift()!;

const html = (
  body: string | Uint8Array<ArrayBuffer>,
  contentType = 'text/html'
) => new Response(body, { headers: { 'content-type': contentType } });

describe('isPrivateAddress', () => {
  it('classifies addresses', () => {
    for (const [address, expected] of [
      ['10.0.0.1', true],
      ['127.0.0.1', true],
      ['169.254.169.254', true],
      ['172.16.0.1', true],
      ['192.168.1.1', true],
      ['::1', true],
      ['::ffff:127.0.0.1', true],
      ['fd00::1', true],
      ['fe80::1', true],
      ['8.8.8.8', false],
      ['172.32.0.1', false],
      ['2606:4700::1111', false],
    ] as const) {
      expect([address, isPrivateAddress(address)]).toEqual([address, expected]);
    }
  });
});

describe('fetchPage', () => {
  it('returns the html and final url', async () => {
    expect(
      await fetchPage('https://foo.com/bar', {
        fetch: respond(
          new Response(null, { status: 301, headers: { location: '/baz' } }),
          html('<p>qux</p>')
        ),
        resolve,
      })
    ).toEqual({ html: '<p>qux</p>', url: 'https://foo.com/baz' });
  });

  it('decodes the declared charset', async () => {
    expect(
      await fetchPage('https://foo.com', {
        fetch: respond(
          html(
            new Uint8Array([0x63, 0x61, 0x66, 0xe9]),
            'text/html; charset=windows-1252'
          )
        ),
        resolve,
      })
    ).toEqual({ html: 'café', url: 'https://foo.com/' });
  });

  it('decodes a meta charset', async () => {
    const bytes = new Uint8Array([
      ...new TextEncoder().encode('<meta charset="iso-8859-1">'),
      0xe9,
    ]);

    expect(
      await fetchPage('https://foo.com', {
        fetch: respond(html(bytes)),
        resolve,
      })
    ).toEqual({
      html: '<meta charset="iso-8859-1">é',
      url: 'https://foo.com/',
    });
  });

  it('rejects invalid pages and destinations', async () => {
    for (const [target, dependencies, message] of [
      ['foo', {}, 'Enter a full URL, like https://example.com/article'],
      ['ftp://foo.com', {}, 'Only http and https URLs are supported'],
      ['http://127.0.0.1', {}, '127.0.0.1 is not a public address'],
      ['http://[::1]', {}, '::1 is not a public address'],
      [
        'https://foo.com',
        { resolve: async () => ['10.0.0.1'] },
        'foo.com is not a public address',
      ],
      [
        'https://foo.com',
        {
          fetch: respond(
            new Response(null, {
              status: 302,
              headers: { location: 'http://169.254.169.254/' },
            })
          ),
          resolve,
        },
        '169.254.169.254 is not a public address',
      ],
      [
        'https://foo.com',
        {
          fetch: respond(
            new Response('', { status: 404, statusText: 'Not Found' })
          ),
          resolve,
        },
        'foo.com returned 404 Not Found',
      ],
      [
        'https://foo.com',
        { fetch: respond(html('{}', 'application/json')), resolve },
        'foo.com returned application/json, not HTML',
      ],
      [
        'https://foo.com',
        {
          fetch: respond(
            html(new Uint8Array([0xff]), 'text/html; charset=utf-8')
          ),
          resolve,
        },
        'The page is not valid utf-8',
      ],
    ] satisfies [string, Dependencies, string][]) {
      expect(fetchPage(target, dependencies)).rejects.toThrow(message);
    }
  });
});

describe('handle', () => {
  it('responds with json', async () => {
    const response = await handle(
      new Request('https://bar.com/api/fetch?url=https://foo.com'),
      { fetch: respond(html('<p>baz</p>')), resolve }
    );

    expect([response.status, await response.json()]).toEqual([
      200,
      { html: '<p>baz</p>', url: 'https://foo.com/' },
    ]);
  });

  it('reports errors', async () => {
    const response = await handle(new Request('https://bar.com/api/fetch'));

    expect([response.status, await response.json()]).toEqual([
      400,
      { error: 'Missing url parameter' },
    ]);
  });
});
