import { useAppearance } from '@/hooks/use-appearance';
import { useArticle } from '@/hooks/use-article';
import { useDocument } from '@/hooks/use-document';
import { useParserOptions } from '@/hooks/use-parser-options';
import { usePersistedState } from '@/hooks/use-persisted-state';
import { useUnclutter } from '@/hooks/use-unclutter';
import { SAMPLE_BASE_URL, sampleNames } from '@/lib/samples';
import { Button } from '@/ui/button';
import { Combobox } from '@/ui/combobox';
import { IconButton } from '@/ui/icon-button';
import { CodeIcon, InfoIcon, SpinnerIcon } from '@/ui/icons';
import { Notice } from '@/ui/notice';
import { Popover } from '@/ui/popover';
import { ReactNode } from 'react';

import { AppearancePanel } from '../components/appearance-panel';
import { DetailsPanel } from '../components/details-panel';
import { Reader } from '../components/reader';
import { SourcePanel } from '../components/source-panel';
import { TopBar } from '../components/top-bar';

const SOURCE_STORAGE_KEY = 'unclutter:source';

const Message = ({ children }: { children: ReactNode }) => (
  <div className='mx-auto max-w-[668px] px-6 pt-14'>{children}</div>
);

const Playground = () => {
  const unclutter = useUnclutter();
  const { appearance, updateAppearance } = useAppearance();
  const { options, updateOptions } = useParserOptions();
  const { document, edit, open, request } = useDocument();

  const [source, setSource] = usePersistedState(SOURCE_STORAGE_KEY, 'closed');
  const sourceOpen = source === 'open';

  const sample =
    document && sampleNames.includes(document.sample) ? document.sample : '';

  const { error, result } = useArticle(
    unclutter.status === 'ready' && document ? unclutter.client : undefined,
    document?.html ?? '',
    SAMPLE_BASE_URL,
    options
  );

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
            Loading the {request.sample} test page
          </p>
        </Message>
      );
    }

    if (request.status === 'error') {
      return (
        <Message>
          <Notice
            tone='danger'
            title={`Couldn’t load the ${request.sample} test page.`}
            actions={
              <Button variant='primary' onClick={() => open(request.sample)}>
                Try again
              </Button>
            }
          >
            {request.message}
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
      />
    );
  };

  return (
    <div className='flex h-dvh flex-col'>
      <TopBar
        center={
          <Combobox
            className='w-full md:w-56'
            empty='No test page found.'
            label='Test page'
            onChange={open}
            options={sampleNames}
            placeholder='Test pages'
            search='Search test pages'
            value={sample}
          />
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
