import { normalizeUrl } from '@/lib/format';
import { loadPage } from '@/lib/load-page';
import { DEFAULT_SAMPLE, loadSample } from '@/lib/samples';
import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useEffect, useRef, useState } from 'react';
import { z } from 'zod';

const DOCUMENT_STORAGE_KEY = 'unclutter:document';

const sourceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('sample'), name: z.string() }),
  z.object({ kind: z.literal('url'), url: z.string() }),
]);

const documentSchema = z.object({ html: z.string(), source: sourceSchema });

export type Document = z.infer<typeof documentSchema>;

export type Source = z.infer<typeof sourceSchema>;

export type Request =
  | { status: 'idle' }
  | { source: Source; status: 'loading' }
  | { message: string; source: Source; status: 'error' };

const parseJson = (text: string): unknown => {
  try {
    return JSON.parse(text);
  } catch {
    return undefined;
  }
};

const readDocument = (): Document | undefined => {
  const saved = readStorage(DOCUMENT_STORAGE_KEY);

  const parsed =
    saved === null ? undefined : documentSchema.safeParse(parseJson(saved));

  return parsed?.success ? parsed.data : undefined;
};

const queryUrl = () => {
  const url = new URLSearchParams(window.location.search).get('url');

  return url ? normalizeUrl(url) : undefined;
};

const setQuery = (source: Source) =>
  window.history.replaceState(
    null,
    '',
    source.kind === 'url'
      ? `?url=${encodeURIComponent(source.url)}`
      : window.location.pathname
  );

const fetchDocument = async (source: Source): Promise<Document> => {
  if (source.kind === 'sample') {
    return { html: await loadSample(source.name), source };
  }

  const page = await loadPage(source.url);

  return { html: page.html, source: { kind: 'url', url: page.url } };
};

export function useDocument() {
  const [initial] = useState(() => {
    const document = readDocument();
    const url = queryUrl();

    const source: Source | undefined = url
      ? { kind: 'url', url }
      : document
        ? undefined
        : { kind: 'sample', name: DEFAULT_SAMPLE };

    return { document, source };
  });

  const [document, setDocument] = useState(initial.document);

  const [request, setRequest] = useState<Request>(
    initial.source
      ? { source: initial.source, status: 'loading' }
      : { status: 'idle' }
  );

  const latest = useRef(0);

  useEffect(() => {
    if (document) {
      writeStorage(DOCUMENT_STORAGE_KEY, JSON.stringify(document));
    }
  }, [document]);

  const load = useCallback(async (source: Source, id: number) => {
    try {
      const document = await fetchDocument(source);

      if (id !== latest.current) return undefined;

      setDocument(document);
      setRequest({ status: 'idle' });
      setQuery(document.source);

      return document.source;
    } catch (error) {
      if (id === latest.current) {
        setRequest({
          message: error instanceof Error ? error.message : String(error),
          source,
          status: 'error',
        });
      }

      return undefined;
    }
  }, []);

  useEffect(() => {
    if (initial.source) load(initial.source, latest.current);
  }, [initial, load]);

  const open = useCallback(
    (source: Source) => {
      const id = ++latest.current;

      setRequest({ source, status: 'loading' });

      return load(source, id);
    },
    [load]
  );

  const openUrl = useCallback(
    (input: string) => open({ kind: 'url', url: normalizeUrl(input) }),
    [open]
  );

  const openSample = useCallback(
    (name: string) => open({ kind: 'sample', name }),
    [open]
  );

  const edit = useCallback((html: string) => {
    setDocument((document) => document && { ...document, html });
  }, []);

  return { document, edit, open, openSample, openUrl, request };
}
