export interface Article {
  byline?: string;
  content: string;
  dir?: string;
  excerpt?: string;
  lang?: string;
  length: number;
  publishedTime?: string;
  siteName?: string;
  textContent: string;
  title: string;
}

export interface Options {
  classesToPreserve: string[];
  keepClasses: boolean;
  linkDensityBias: number;
  maxElements?: number;
  minTextLength: number;
  nTopCandidates: number;
  useJsonLd: boolean;
}

export type ParseResult = { duration: number; readable: boolean } & (
  { status: 'ok'; article: Article } | { status: 'error'; message: string }
);
