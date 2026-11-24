import type { Options, ParseResult } from '../types';

export type UnclutterOperation =
  | { method: 'initialize' }
  | {
      method: 'parse';
      html: string;
      baseUrl: string | undefined;
      options: Options;
    };

export type UnclutterRequest = UnclutterOperation & { id: number };

export type UnclutterResult =
  | { method: 'initialize'; result: undefined }
  | { method: 'parse'; result: ParseResult };

export type UnclutterResponse = { id: number } & (
  UnclutterResult | { error: string }
);
