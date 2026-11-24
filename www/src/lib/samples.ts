import blog from '../../samples/blog.html?raw';
import minimal from '../../samples/minimal.html?raw';
import news from '../../samples/news.html?raw';

export const samples = [
  { name: 'news', code: news },
  { name: 'blog', code: blog },
  { name: 'minimal', code: minimal },
].map((sample) => ({ ...sample, code: sample.code.trim() }));

export function getSample(name: string) {
  return samples.find((sample) => sample.name === name) ?? samples[0];
}
