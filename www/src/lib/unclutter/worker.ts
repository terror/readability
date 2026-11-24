import init, { isReadable, parse } from '../unclutter-wasm/unclutter_wasm';
import type { UnclutterRequest, UnclutterResponse } from './protocol';

let initialization: ReturnType<typeof init> | undefined;

function execute(request: UnclutterRequest): UnclutterResponse {
  const { id } = request;

  switch (request.method) {
    case 'initialize':
      return { id, method: request.method, result: undefined };
    case 'parse': {
      const start = performance.now();

      const readable = isReadable(request.html);

      try {
        const article = parse(request.html, request.baseUrl, request.options);

        return {
          id,
          method: request.method,
          result: {
            article,
            duration: performance.now() - start,
            readable,
            status: 'ok',
          },
        };
      } catch (error) {
        return {
          id,
          method: request.method,
          result: {
            duration: performance.now() - start,
            message: error instanceof Error ? error.message : String(error),
            readable,
            status: 'error',
          },
        };
      }
    }
  }
}

self.addEventListener(
  'message',
  async (event: MessageEvent<UnclutterRequest>) => {
    try {
      await (initialization ??= init().catch((error) => {
        initialization = undefined;
        throw error;
      }));

      self.postMessage(execute(event.data));
    } catch (error) {
      self.postMessage({
        id: event.data.id,
        error: error instanceof Error ? error.message : String(error),
      } satisfies UnclutterResponse);
    }
  }
);
