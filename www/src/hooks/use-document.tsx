import { DEFAULT_SAMPLE, loadSample } from '@/lib/samples';
import { readStorage, writeStorage } from '@/lib/storage';
import { useCallback, useEffect, useRef, useState } from 'react';
import { z } from 'zod';

const DOCUMENT_STORAGE_KEY = 'unclutter:document';

const documentSchema = z.object({ html: z.string(), sample: z.string() });

export type Document = z.infer<typeof documentSchema>;

export type Request =
  | { status: 'idle' }
  | { sample: string; status: 'loading' }
  | { message: string; sample: string; status: 'error' };

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

export function useDocument() {
  const [initial] = useState(readDocument);

  const [document, setDocument] = useState(initial);

  const [request, setRequest] = useState<Request>(
    initial ? { status: 'idle' } : { sample: DEFAULT_SAMPLE, status: 'loading' }
  );

  const latest = useRef(0);

  useEffect(() => {
    if (document) {
      writeStorage(DOCUMENT_STORAGE_KEY, JSON.stringify(document));
    }
  }, [document]);

  const load = useCallback(
    (sample: string, id: number) =>
      loadSample(sample).then(
        (html) => {
          if (id !== latest.current) return;

          setDocument({ html, sample });
          setRequest({ status: 'idle' });
        },
        (error) => {
          if (id !== latest.current) return;

          setRequest({
            message: error instanceof Error ? error.message : String(error),
            sample,
            status: 'error',
          });
        }
      ),
    []
  );

  useEffect(() => {
    if (!initial) load(DEFAULT_SAMPLE, latest.current);
  }, [initial, load]);

  const open = useCallback(
    (sample: string) => {
      const id = ++latest.current;

      setRequest({ sample, status: 'loading' });

      return load(sample, id);
    },
    [load]
  );

  const edit = useCallback((html: string) => {
    setDocument((document) => document && { ...document, html });
  }, []);

  return { document, edit, open, request };
}
