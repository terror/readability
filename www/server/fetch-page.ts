import { lookup } from 'node:dns/promises';
import { isIP } from 'node:net';

const MAX_BYTES = 5 * 1024 * 1024;
const MAX_REDIRECTS = 5;
const TIMEOUT = 10_000;
const USER_AGENT =
  'Mozilla/5.0 (compatible; unclutter-playground; +https://github.com/terror/unclutter)';

export class FetchPageError extends Error {
  constructor(
    message: string,
    readonly status: number
  ) {
    super(message);
  }
}

type Resolve = (hostname: string) => Promise<string[]>;

export interface Dependencies {
  fetch?: (url: URL, init: RequestInit) => Promise<Response>;
  resolve?: Resolve;
}

const resolveHost: Resolve = async (hostname) =>
  (await lookup(hostname, { all: true })).map(({ address }) => address);

const isPrivateIpv4 = (address: string) => {
  const [a, b] = address.split('.').map(Number);

  return (
    a === 0 ||
    a === 10 ||
    a === 127 ||
    (a === 100 && b >= 64 && b <= 127) ||
    (a === 169 && b === 254) ||
    (a === 172 && b >= 16 && b <= 31) ||
    (a === 192 && b === 168) ||
    a >= 224
  );
};

export function isPrivateAddress(address: string): boolean {
  if (isIP(address) === 4) return isPrivateIpv4(address);

  const normalized = address.toLowerCase();

  const mapped = /^::ffff:(\d+\.\d+\.\d+\.\d+)$/.exec(normalized);

  if (mapped) return isPrivateIpv4(mapped[1]);

  return (
    normalized === '::' ||
    normalized === '::1' ||
    /^f[cd]/.test(normalized) ||
    /^fe[89ab]/.test(normalized) ||
    /^ff/.test(normalized)
  );
}

async function checkHost(url: URL, resolve: Resolve) {
  if (url.protocol !== 'http:' && url.protocol !== 'https:') {
    throw new FetchPageError('Only http and https URLs are supported', 400);
  }

  const hostname = url.hostname.replace(/^\[|\]$/g, '');

  const addresses = isIP(hostname)
    ? [hostname]
    : await resolve(hostname).catch(() => {
        throw new FetchPageError(`Could not resolve ${hostname}`, 502);
      });

  if (addresses.length === 0 || addresses.some(isPrivateAddress)) {
    throw new FetchPageError(`${hostname} is not a public address`, 400);
  }
}

async function readLimited(response: Response): Promise<Uint8Array> {
  const reader = response.body?.getReader();

  if (!reader) return new Uint8Array();

  const chunks: Uint8Array[] = [];
  let size = 0;

  for (;;) {
    const { done, value } = await reader.read();

    if (done) break;

    size += value.byteLength;

    if (size > MAX_BYTES) {
      await reader.cancel();
      throw new FetchPageError('The page is larger than 5 MB', 413);
    }

    chunks.push(value);
  }

  const bytes = new Uint8Array(size);
  let offset = 0;

  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }

  return bytes;
}

function decode(bytes: Uint8Array, contentType: string): string {
  const label =
    /charset=["']?([\w-]+)/i.exec(contentType)?.[1] ??
    /<meta[^>]+charset=["']?([\w-]+)/i.exec(
      new TextDecoder('latin1').decode(bytes.subarray(0, 1024))
    )?.[1] ??
    'utf-8';

  const decoder = (() => {
    try {
      return new TextDecoder(label, { fatal: true });
    } catch {
      return new TextDecoder('utf-8', { fatal: true });
    }
  })();

  try {
    return decoder.decode(bytes);
  } catch {
    throw new FetchPageError(`The page is not valid ${decoder.encoding}`, 502);
  }
}

export async function fetchPage(
  target: string,
  { fetch: request = fetch, resolve = resolveHost }: Dependencies = {}
): Promise<{ html: string; url: string }> {
  let url: URL;

  try {
    url = new URL(target);
  } catch {
    throw new FetchPageError(
      'Enter a full URL, like https://example.com/article',
      400
    );
  }

  const signal = AbortSignal.timeout(TIMEOUT);

  for (let redirects = 0; ; redirects++) {
    await checkHost(url, resolve);

    let response: Response;

    try {
      response = await request(url, {
        headers: {
          accept: 'text/html,application/xhtml+xml',
          'user-agent': USER_AGENT,
        },
        redirect: 'manual',
        signal,
      });
    } catch {
      throw new FetchPageError(
        signal.aborted
          ? `${url.hostname} took too long to respond`
          : `Could not reach ${url.hostname}`,
        502
      );
    }

    const location = response.headers.get('location');

    if (response.status >= 300 && response.status < 400 && location) {
      if (redirects === MAX_REDIRECTS) {
        throw new FetchPageError('The page redirected too many times', 502);
      }

      url = new URL(location, url);
      continue;
    }

    if (!response.ok) {
      throw new FetchPageError(
        `${url.hostname} returned ${[response.status, response.statusText].filter(Boolean).join(' ')}`,
        502
      );
    }

    const contentType = response.headers.get('content-type') ?? '';

    if (
      contentType &&
      !/text\/html|application\/xhtml\+xml/i.test(contentType)
    ) {
      throw new FetchPageError(
        `${url.hostname} returned ${contentType.split(';')[0]}, not HTML`,
        415
      );
    }

    return {
      html: decode(await readLimited(response), contentType),
      url: url.href,
    };
  }
}

export async function handle(
  request: Request,
  dependencies?: Dependencies
): Promise<Response> {
  const target = new URL(request.url).searchParams.get('url');

  if (!target) {
    return Response.json({ error: 'Missing url parameter' }, { status: 400 });
  }

  try {
    return Response.json(await fetchPage(target, dependencies), {
      headers: { 'cache-control': 'public, s-maxage=300' },
    });
  } catch (error) {
    return error instanceof FetchPageError
      ? Response.json({ error: error.message }, { status: error.status })
      : Response.json({ error: 'Failed to fetch the page' }, { status: 500 });
  }
}
