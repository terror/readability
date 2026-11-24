const pages = import.meta.glob<string>(
  '../../../submodules/readability/test/test-pages/*/source.html',
  { import: 'default', query: '?raw' }
);

export const DEFAULT_SAMPLE = 'wikipedia';

export const SAMPLE_BASE_URL = 'http://fakehost/test/page.html';

export const samples = Object.entries(pages)
  .map(([path, load]) => ({
    load,
    name: /test-pages\/([^/]+)\//.exec(path)?.[1] ?? path,
  }))
  .sort((a, b) => a.name.localeCompare(b.name));

export async function loadSample(name: string): Promise<string> {
  const sample = samples.find((sample) => sample.name === name);

  if (!sample) throw new Error(`There is no test page named ${name}.`);

  return sample.load();
}
