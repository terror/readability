import { normalizeUrl } from '@/lib/format';
import { loadPage } from '@/lib/load-page';
import { getSample, samples } from '@/lib/samples';
import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useEffect, useRef, useState } from 'react';
import { z } from 'zod';

const DOCUMENT_STORAGE_KEY = 'unclutter:document';

const documentSchema = z.object({
  html: z.string(),
  source: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('sample'), name: z.string() }),
    z.object({ kind: z.literal('url'), url: z.string() }),
  ]),
});

export type Document = z.infer<typeof documentSchema>;

export type Request =
  | { status: 'idle' }
  | { status: 'loading'; url: string }
  | { message: string; status: 'error'; url: string };

const sampleDocument = (name: string): Document => {
  const sample = getSample(name);

  return { html: sample.code, source: { kind: 'sample', name: sample.name } };
};

const parseJson = (text: string): unknown => {
  try {
    return JSON.parse(text);
  } catch {
    return undefined;
  }
};

const readDocument = (): Document => {
  const saved = readStorage(DOCUMENT_STORAGE_KEY);

  const parsed =
    saved === null ? undefined : documentSchema.safeParse(parseJson(saved));

  return parsed?.success ? parsed.data : sampleDocument(samples[0].name);
};

const setQuery = (url: string | undefined) =>
  window.history.replaceState(
    null,
    '',
    url ? `?url=${encodeURIComponent(url)}` : window.location.pathname
  );

const queryUrl = () => {
  const url = new URLSearchParams(window.location.search).get('url');

  return url ? normalizeUrl(url) : undefined;
};

export function useDocument() {
  const [document, setDocument] = useState(readDocument);

  const [request, setRequest] = useState<Request>(() => {
    const url = queryUrl();

    return url ? { status: 'loading', url } : { status: 'idle' };
  });

  const latest = useRef(0);

  useEffect(() => {
    writeStorage(DOCUMENT_STORAGE_KEY, JSON.stringify(document));
  }, [document]);

  const load = useCallback(async (url: string, id: number) => {
    try {
      const page = await loadPage(url);

      if (id !== latest.current) return undefined;

      setDocument({ html: page.html, source: { kind: 'url', url: page.url } });
      setRequest({ status: 'idle' });
      setQuery(page.url);

      return page.url;
    } catch (error) {
      if (id === latest.current) {
        setRequest({
          message: error instanceof Error ? error.message : String(error),
          status: 'error',
          url,
        });
      }

      return undefined;
    }
  }, []);

  useEffect(() => {
    const url = queryUrl();

    if (url) load(url, latest.current);
  }, [load]);

  const openUrl = useCallback(
    (input: string) => {
      const url = normalizeUrl(input);
      const id = ++latest.current;

      setRequest({ status: 'loading', url });

      return load(url, id);
    },
    [load]
  );

  const openSample = useCallback((name: string) => {
    latest.current++;
    setRequest({ status: 'idle' });
    setDocument(sampleDocument(name));
    setQuery(undefined);
  }, []);

  const edit = useCallback((html: string) => {
    setDocument((document) => ({ ...document, html }));
  }, []);

  return { document, edit, openSample, openUrl, request };
}
