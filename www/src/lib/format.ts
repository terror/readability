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

export function readingTime(text: string): string {
  const words = text.split(/\s+/).filter(Boolean).length;

  return `${Math.max(1, Math.round(words / 230))} min`;
}
