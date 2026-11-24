import type { ParserOptions } from '@/lib/parser-options';
import type { ParseResult } from '@/lib/types';
import type { UnclutterClient } from '@/lib/unclutter/client';
import { useEffect, useState } from 'react';

const PARSE_DELAY = 150;

export function useArticle(
  client: UnclutterClient | undefined,
  html: string,
  baseUrl: string | undefined,
  options: ParserOptions
) {
  const [state, setState] = useState<{
    error?: string;
    result?: ParseResult;
  }>({});

  useEffect(() => {
    if (!client) return;

    let active = true;

    const timeout = setTimeout(() => {
      client.parse(html, baseUrl, options).then(
        (result) => {
          if (active) setState({ result });
        },
        (error) => {
          if (active) {
            setState({
              error: error instanceof Error ? error.message : String(error),
            });
          }
        }
      );
    }, PARSE_DELAY);

    return () => {
      active = false;
      clearTimeout(timeout);
    };
  }, [baseUrl, client, html, options]);

  return state;
}
