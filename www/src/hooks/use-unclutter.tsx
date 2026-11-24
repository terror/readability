import { UnclutterClient } from '@/lib/unclutter/client';
import { useEffect, useState } from 'react';

type UnclutterState =
  | { status: 'loading' }
  | { status: 'error'; error: string }
  | { status: 'ready'; client: UnclutterClient };

export function useUnclutter(): UnclutterState {
  const [state, setState] = useState<UnclutterState>({ status: 'loading' });

  useEffect(() => {
    let active = true;

    const client = new UnclutterClient();

    client.initialize().then(
      () => {
        if (active) setState({ status: 'ready', client });
      },
      (error) => {
        client.dispose();

        if (active) {
          setState({
            status: 'error',
            error: `Failed to initialize playground: ${error instanceof Error ? error.message : String(error)}`,
          });
        }
      }
    );

    return () => {
      active = false;
      client.dispose();
    };
  }, []);

  return state;
}
