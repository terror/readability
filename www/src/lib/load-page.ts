export interface Page {
  html: string;
  url: string;
}

export async function loadPage(
  url: string,
  request: (input: string) => Promise<Response> = fetch
): Promise<Page> {
  let response: Response;

  try {
    response = await request(`/api/fetch?url=${encodeURIComponent(url)}`);
  } catch {
    throw new Error('Could not reach the playground server.');
  }

  const body = (await response.json().catch(() => undefined)) as
    Partial<Page & { error: string }> | undefined;

  if (!response.ok || body?.html === undefined || body.url === undefined) {
    throw new Error(
      body?.error ?? `The playground server returned ${response.status}.`
    );
  }

  return { html: body.html, url: body.url };
}
