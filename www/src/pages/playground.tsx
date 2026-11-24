import { useAppearance } from '@/hooks/use-appearance';
import { useArticle } from '@/hooks/use-article';
import { useDocument } from '@/hooks/use-document';
import { useParserOptions } from '@/hooks/use-parser-options';
import { usePersistedState } from '@/hooks/use-persisted-state';
import { useUnclutter } from '@/hooks/use-unclutter';
import { hostname } from '@/lib/format';
import { SAMPLE_BASE_URL, samples } from '@/lib/samples';
import { Button } from '@/ui/button';
import { IconButton } from '@/ui/icon-button';
import { CodeIcon, InfoIcon, SpinnerIcon } from '@/ui/icons';
import { LinkField } from '@/ui/link-field';
import { Notice } from '@/ui/notice';
import { Popover } from '@/ui/popover';
import { Select } from '@/ui/select';
import { ReactNode, useState } from 'react';

import { AppearancePanel } from '../components/appearance-panel';
import { DetailsPanel } from '../components/details-panel';
import { Reader } from '../components/reader';
import { SourcePanel } from '../components/source-panel';
import { TopBar } from '../components/top-bar';
import type { Source } from '../hooks/use-document';

const SOURCE_STORAGE_KEY = 'unclutter:source';

const describe = (source: Source) =>
  source.kind === 'url'
    ? (hostname(source.url) ?? source.url)
    : `the ${source.name} test page`;

const Message = ({ children }: { children: ReactNode }) => (
  <div className='mx-auto max-w-[668px] px-6 pt-14'>{children}</div>
);

const Playground = () => {
  const unclutter = useUnclutter();
  const { appearance, updateAppearance } = useAppearance();
  const { options, updateOptions } = useParserOptions();
  const { document, edit, open, openSample, openUrl, request } = useDocument();

  const [source, setSource] = usePersistedState(SOURCE_STORAGE_KEY, 'closed');
  const sourceOpen = source === 'open';

  const [link, setLink] = useState(
    () =>
      new URLSearchParams(window.location.search).get('url') ??
      (document?.source.kind === 'url' ? document.source.url : '')
  );

  const url = document?.source.kind === 'url' ? document.source.url : undefined;

  const selected =
    document?.source.kind === 'sample' ? document.source.name : '';

  const sample = samples.some(({ name }) => name === selected) ? selected : '';

  const { error, result } = useArticle(
    unclutter.status === 'ready' && document ? unclutter.client : undefined,
    document?.html ?? '',
    url ?? SAMPLE_BASE_URL,
    options
  );

  const read = async (value: string) => {
    const final = await openUrl(value);

    if (final?.kind === 'url') setLink(final.url);
  };

  const content = () => {
    if (unclutter.status === 'error') {
      return (
        <Message>
          <Notice tone='danger' title='The parser failed to load.'>
            {unclutter.error}
          </Notice>
        </Message>
      );
    }

    if (request.status === 'loading') {
      return (
        <Message>
          <p className='text-ink-3 flex items-center gap-2 text-sm'>
            <SpinnerIcon size={16} />
            Loading {describe(request.source)}
          </p>
        </Message>
      );
    }

    if (request.status === 'error') {
      return (
        <Message>
          <Notice
            tone='danger'
            title={`Couldn’t load ${describe(request.source)}.`}
            actions={
              <>
                <Button variant='primary' onClick={() => open(request.source)}>
                  Try again
                </Button>
                {request.source.kind === 'url' && (
                  <Button onClick={() => setSource('open')}>Paste HTML</Button>
                )}
              </>
            }
          >
            {request.message}
            {request.source.kind === 'url' &&
              ' Some sites block automated requests; you can save the page and paste its HTML instead.'}
          </Notice>
        </Message>
      );
    }

    if (error || result?.status === 'error') {
      return (
        <Message>
          <Notice tone='danger' title='Couldn’t extract an article.'>
            {error ?? (result?.status === 'error' ? result.message : '')}
          </Notice>
        </Message>
      );
    }

    if (result?.status !== 'ok') return null;

    return (
      <Reader
        appearance={appearance}
        article={result.article}
        duration={result.duration}
        readable={result.readable}
        url={url}
      />
    );
  };

  return (
    <div className='flex h-dvh flex-col'>
      <TopBar
        field={
          <div className='flex gap-2'>
            <div className='min-w-0 flex-1'>
              <LinkField
                value={link}
                onChange={setLink}
                onSubmit={read}
                loading={
                  request.status === 'loading' && request.source.kind === 'url'
                }
              />
            </div>
            <Select
              aria-label='Test page'
              className='w-36 shrink-0 sm:w-44'
              value={sample}
              onChange={(event) => {
                setLink('');
                openSample(event.target.value);
              }}
            >
              <option value='' disabled>
                Test pages
              </option>
              {samples.map(({ name }) => (
                <option key={name} value={name}>
                  {name}
                </option>
              ))}
            </Select>
          </div>
        }
        actions={
          <>
            <IconButton
              label='Source'
              active={sourceOpen}
              aria-pressed={sourceOpen}
              onClick={() => setSource(sourceOpen ? 'closed' : 'open')}
            >
              <CodeIcon />
            </IconButton>
            <Popover
              label='Appearance'
              icon={
                <span className='font-read text-[15px] font-medium'>Aa</span>
              }
            >
              <AppearancePanel
                appearance={appearance}
                updateAppearance={updateAppearance}
              />
            </Popover>
            <Popover label='Details' icon={<InfoIcon />}>
              <DetailsPanel
                options={options}
                result={result}
                updateOptions={updateOptions}
              />
            </Popover>
          </>
        }
      />
      <div className='flex min-h-0 flex-1'>
        {sourceOpen && document && (
          <div className='border-line min-w-0 flex-1 md:flex-none md:basis-[44%] md:border-r'>
            <SourcePanel
              dark={appearance.theme === 'dark'}
              html={document.html}
              onChange={edit}
              output={
                result?.status === 'ok' ? result.article.content : undefined
              }
            />
          </div>
        )}
        <main
          className={`min-w-0 flex-1 overflow-y-auto ${sourceOpen ? 'max-md:hidden' : ''}`}
        >
          {content()}
        </main>
      </div>
    </div>
  );
};

export default Playground;
