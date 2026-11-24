import type { Appearance } from '@/lib/appearance';
import { formatDate, readingTime } from '@/lib/format';
import { sanitize } from '@/lib/sanitize';
import type { Article } from '@/lib/types';
import { Notice } from '@/ui/notice';
import { CSSProperties, useMemo } from 'react';

interface ReaderProps {
  appearance: Appearance;
  article: Article;
  readable: boolean;
}

export const Reader = ({ appearance, article, readable }: ReaderProps) => {
  const content = useMemo(() => sanitize(article.content), [article.content]);

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
    </article>
  );
};
