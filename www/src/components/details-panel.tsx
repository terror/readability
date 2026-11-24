import { formatDate } from '@/lib/format';
import { ParserOptions, defaultParserOptions } from '@/lib/parser-options';
import type { ParseResult } from '@/lib/types';
import { ParsedInput } from '@/ui/parsed-input';
import { Row, Section } from '@/ui/section';
import { Status } from '@/ui/status';
import { Switch } from '@/ui/switch';

const parseCount = (text: string, minimum: number) => {
  const value = Number(text);

  return text.trim() !== '' && Number.isInteger(value) && value >= minimum
    ? { value }
    : undefined;
};

const parseNumber = (text: string) => {
  const value = Number(text);

  return text.trim() !== '' && Number.isFinite(value) ? { value } : undefined;
};

const parseLimit = (text: string) =>
  text.trim() === '' ? { value: null } : parseCount(text, 1);

const parseClasses = (text: string) => ({
  value: text.split(/[\s,]+/).filter(Boolean),
});

interface DetailsPanelProps {
  options: ParserOptions;
  result?: ParseResult;
  updateOptions: (options: Partial<ParserOptions>) => void;
}

export const DetailsPanel = ({
  options,
  result,
  updateOptions,
}: DetailsPanelProps) => {
  const changed = Object.entries(defaultParserOptions).some(
    ([key, value]) =>
      JSON.stringify(options[key as keyof ParserOptions]) !==
      JSON.stringify(value)
  );

  const article = result?.status === 'ok' ? result.article : undefined;

  const metadata: [string, string | undefined][] = article
    ? [
        ['Byline', article.byline],
        ['Site', article.siteName],
        [
          'Published',
          article.publishedTime && formatDate(article.publishedTime),
        ],
        ['Language', article.lang],
        ['Direction', article.dir],
        ['Length', `${article.length.toLocaleString()} characters`],
        ['Excerpt', article.excerpt],
      ]
    : [];

  return (
    <div className='flex flex-col gap-6'>
      <div className='flex items-center justify-between'>
        <h2 className='text-[17px] font-semibold'>Details</h2>
        {result && (
          <Status
            tone={
              result.status === 'error'
                ? 'danger'
                : result.readable
                  ? 'ok'
                  : 'warn'
            }
          >
            {result.status === 'error'
              ? 'Failed'
              : result.readable
                ? 'Readable'
                : 'Not an article'}{' '}
            · {result.duration.toFixed(1)} ms
          </Status>
        )}
      </div>
      {article && (
        <dl className='flex flex-col text-sm'>
          {metadata.map(([label, value]) => (
            <div
              key={label}
              className='border-line flex justify-between gap-6 border-b py-2.5 last:border-b-0'
            >
              <dt className='text-ink-3 shrink-0'>{label}</dt>
              <dd
                className={`min-w-0 text-right break-words ${value ? '' : 'text-ink-3'}`}
              >
                {value || 'None'}
              </dd>
            </div>
          ))}
        </dl>
      )}
      <Section
        title='Parser options'
        action={
          <button
            type='button'
            disabled={!changed}
            onClick={() => updateOptions(defaultParserOptions)}
            className='text-ink-3 hover:text-ink cursor-pointer text-xs disabled:cursor-default disabled:opacity-40'
          >
            Reset
          </button>
        }
      >
        <div className='flex flex-col'>
          <Row label='Read JSON-LD metadata'>
            <Switch
              checked={options.useJsonLd}
              onChange={(event) =>
                updateOptions({ useJsonLd: event.target.checked })
              }
            />
          </Row>
          <Row label='Keep all classes'>
            <Switch
              checked={options.keepClasses}
              onChange={(event) =>
                updateOptions({ keepClasses: event.target.checked })
              }
            />
          </Row>
          <Row label='Classes to keep'>
            <ParsedInput
              className='w-32'
              disabled={options.keepClasses}
              placeholder='None'
              value={options.classesToPreserve}
              format={(value) => value.join(' ')}
              parse={parseClasses}
              onValueChange={(classesToPreserve) =>
                updateOptions({ classesToPreserve })
              }
            />
          </Row>
          <Row label='Min text length'>
            <ParsedInput
              inputMode='numeric'
              value={options.minTextLength}
              format={(value) => value.toString()}
              parse={(text) => parseCount(text, 0)}
              onValueChange={(minTextLength) =>
                updateOptions({ minTextLength })
              }
            />
          </Row>
          <Row label='Top candidates'>
            <ParsedInput
              inputMode='numeric'
              value={options.nTopCandidates}
              format={(value) => value.toString()}
              parse={(text) => parseCount(text, 1)}
              onValueChange={(nTopCandidates) =>
                updateOptions({ nTopCandidates })
              }
            />
          </Row>
          <Row label='Link density bias'>
            <ParsedInput
              inputMode='decimal'
              value={options.linkDensityBias}
              format={(value) => value.toString()}
              parse={parseNumber}
              onValueChange={(linkDensityBias) =>
                updateOptions({ linkDensityBias })
              }
            />
          </Row>
          <Row label='Max elements'>
            <ParsedInput
              inputMode='numeric'
              placeholder='No limit'
              value={options.maxElements}
              format={(value) => value?.toString() ?? ''}
              parse={parseLimit}
              onValueChange={(maxElements) => updateOptions({ maxElements })}
            />
          </Row>
        </div>
      </Section>
    </div>
  );
};
