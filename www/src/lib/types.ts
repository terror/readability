import type { Article } from './unclutter-wasm/unclutter_wasm';

export type { Article, Options } from './unclutter-wasm/unclutter_wasm';

export type ParseResult = { duration: number; readable: boolean } & (
  { status: 'ok'; article: Article } | { status: 'error'; message: string }
);
