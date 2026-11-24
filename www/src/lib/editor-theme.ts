import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { EditorView } from '@codemirror/view';
import { tags } from '@lezer/highlight';

const theme = (dark: boolean) =>
  EditorView.theme(
    {
      '&': {
        backgroundColor: 'transparent',
        color: 'var(--ink-2)',
        fontSize: '12.5px',
        height: '100%',
      },
      '&.cm-focused': {
        outline: 'none',
      },
      '&.cm-focused > .cm-scroller > .cm-selectionLayer .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection':
        {
          backgroundColor: 'var(--accent-wash)',
        },
      '.cm-content': {
        caretColor: 'var(--accent)',
        padding: '8px 0 48px',
      },
      '.cm-cursor, .cm-dropCursor': {
        borderLeftColor: 'var(--accent)',
      },
      '.cm-gutters': {
        display: 'none',
      },
      '.cm-line': {
        padding: '0 20px',
      },
      '.cm-matchingBracket, .cm-nonmatchingBracket': {
        backgroundColor: 'transparent',
        color: 'inherit',
      },
      '.cm-scroller': {
        fontFamily: 'var(--font-code)',
        lineHeight: '21px',
      },
    },
    { dark }
  );

const highlight = syntaxHighlighting(
  HighlightStyle.define([
    { tag: tags.tagName, color: 'var(--ink)' },
    {
      tag: [
        tags.angleBracket,
        tags.attributeName,
        tags.punctuation,
        tags.propertyName,
      ],
      color: 'var(--ink-3)',
    },
    { tag: [tags.attributeValue, tags.string], color: 'var(--accent)' },
    {
      tag: [tags.comment, tags.documentMeta, tags.processingInstruction],
      color: 'var(--ink-3)',
      fontStyle: 'italic',
    },
  ])
);

export const lightTheme = [theme(false), highlight];
export const darkTheme = [theme(true), highlight];
