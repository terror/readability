import type { Appearance } from '@/lib/appearance';
import { formatDate, hostname, readingTime } from '@/lib/format';
import { sanitize } from '@/lib/sanitize';
import type { Article } from '@/lib/types';
import { Notice } from '@/ui/notice';
import { Status } from '@/ui/status';
import { CSSProperties, useMemo } from 'react';

interface ReaderProps {
  appearance: Appearance;
  article: Article;
  duration: number;
  readable: boolean;
  url?: string;
}

export const Reader = ({
  appearance,
  article,
  duration,
  readable,
  url,
}: ReaderProps) => {
  const content = useMemo(() => sanitize(article.content), [article.content]);

  const host = url ? hostname(url) : undefined;

  const meta = [
    article.byline,
    article.siteName,
    article.publishedTime && formatDate(article.publishedTime),
    readingTime(article.textContent),
  ].filter(Boolean);

  const font = appearance.typeface === 'serif' ? 'font-read' : 'font-ui';

  return (
    <article
      className='mx-auto px-6 pt-10 pb-24 md:pt-14'
      style={{ maxWidth: appearance.width === 'wide' ? 808 : 668 }}
    >
      {!readable && (
        <div className='mb-10'>
          <Notice tone='warn' title='This page may not be an article.'>
            The result can be incomplete. Lowering the minimum text length in
            Details can help.
          </Notice>
        </div>
      )}
      {host && (
        <p className='text-ink-3 mb-5 text-[13px] font-medium'>{host}</p>
      )}
      {article.title && (
        <h1
          className={`${font} leading-[1.15] font-medium tracking-[-0.015em] text-balance`}
          style={{ fontSize: appearance.textSize * 2 }}
        >
          {article.title}
        </h1>
      )}
      <p className='text-ink-2 mt-4 text-sm'>{meta.join(' · ')}</p>
      <div
        dir={article.dir}
        lang={article.lang}
        className={`reader-content mt-10 [&>:last-child>:last-child>:last-child]:mb-0 ${font}`}
        style={{ '--reader-size': `${appearance.textSize}px` } as CSSProperties}
        dangerouslySetInnerHTML={{ __html: content }}
      />
      <footer className='border-line text-ink-3 mt-14 flex flex-wrap gap-x-4 gap-y-2 border-t pt-5 text-[13px] tabular-nums'>
        <Status tone={readable ? 'ok' : 'warn'}>
          {readable ? 'Readable' : 'Probably not an article'}
        </Status>
        <span>{article.length.toLocaleString()} characters</span>
        <span>Extracted in {duration.toFixed(1)} ms</span>
      </footer>
    </article>
  );
};
