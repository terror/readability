export function formatDate(value: string): string {
  const date = new Date(value);

  return Number.isNaN(date.getTime())
    ? value
    : date.toLocaleDateString('en-US', {
        day: 'numeric',
        month: 'long',
        timeZone: 'UTC',
        year: 'numeric',
      });
}

export function hostname(url: string): string | undefined {
  try {
    return new URL(url).hostname.replace(/^www\./, '');
  } catch {
    return undefined;
  }
}

export function normalizeUrl(input: string): string {
  const url = input.trim();

  return /^[a-z][a-z\d+.-]*:/i.test(url) ? url : `https://${url}`;
}

export function readingTime(text: string): string {
  const words = text.split(/\s+/).filter(Boolean).length;

  return `${Math.max(1, Math.round(words / 230))} min`;
}
