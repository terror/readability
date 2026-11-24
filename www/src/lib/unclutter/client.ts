import { type ParserOptions, toOptions } from '../parser-options';
import type { ParseResult } from '../types';
import type {
  UnclutterOperation,
  UnclutterRequest,
  UnclutterResponse,
  UnclutterResult,
} from './protocol';

export class UnclutterClient {
  private id = 0;
  private error: Error | undefined;

  private pending = new Map<
    number,
    {
      resolve: (response: UnclutterResult) => void;
      reject: (error: Error) => void;
    }
  >();

  private worker = new Worker(new URL('./worker.ts', import.meta.url), {
    type: 'module',
  });

  constructor() {
    this.worker.addEventListener(
      'message',
      (event: MessageEvent<UnclutterResponse>) => {
        const pending = this.pending.get(event.data.id);

        this.pending.delete(event.data.id);

        if ('error' in event.data) {
          pending?.reject(new Error(event.data.error));
        } else {
          pending?.resolve(event.data);
        }
      }
    );

    this.worker.addEventListener('error', (event) => {
      this.stop(new Error(event.message));
    });

    this.worker.addEventListener('messageerror', () => {
      this.stop(new Error('Failed to deserialize parse response'));
    });
  }

  dispose(): void {
    this.stop(new Error('Unclutter client disposed'));
  }

  initialize(): Promise<void> {
    return this.request({ method: 'initialize' });
  }

  parse(
    html: string,
    baseUrl: string | undefined,
    options: ParserOptions
  ): Promise<ParseResult> {
    return this.request({
      method: 'parse',
      html,
      baseUrl,
      options: toOptions(options),
    });
  }

  private request(
    request: Extract<UnclutterOperation, { method: 'initialize' }>
  ): Promise<undefined>;
  private request(
    request: Extract<UnclutterOperation, { method: 'parse' }>
  ): Promise<ParseResult>;
  private request(
    request: UnclutterOperation
  ): Promise<UnclutterResult['result']> {
    if (this.error) return Promise.reject(this.error);

    const id = this.id++;

    return new Promise((resolve, reject) => {
      this.pending.set(id, {
        resolve: (response) => {
          if (response.method === request.method) {
            resolve(response.result);
          } else {
            reject(
              new Error(
                `Unexpected parse response: expected ${request.method}, received ${response.method}`
              )
            );
          }
        },
        reject,
      });

      try {
        this.worker.postMessage({ id, ...request } satisfies UnclutterRequest);
      } catch (error) {
        this.pending.delete(id);
        reject(error);
      }
    });
  }

  private stop(error: Error): void {
    this.error ??= error;

    this.worker.terminate();

    for (const pending of this.pending.values()) {
      pending.reject(this.error);
    }

    this.pending.clear();
  }
}
