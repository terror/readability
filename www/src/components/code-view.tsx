import { darkTheme, lightTheme } from '@/lib/editor-theme';
import { html } from '@codemirror/lang-html';
import { EditorView } from '@codemirror/view';
import CodeMirror from '@uiw/react-codemirror';
import { useMemo } from 'react';

interface CodeViewProps {
  dark: boolean;
  label: string;
  onChange?: (value: string) => void;
  readOnly?: boolean;
  value: string;
}

const basicSetup = {
  autocompletion: false,
  closeBrackets: false,
  foldGutter: false,
  highlightActiveLine: false,
  highlightActiveLineGutter: false,
  highlightSelectionMatches: false,
  lineNumbers: false,
};

export const CodeView = ({
  dark,
  label,
  onChange,
  readOnly = false,
  value,
}: CodeViewProps) => {
  const extensions = useMemo(
    () => [
      EditorView.lineWrapping,
      EditorView.contentAttributes.of({ 'aria-label': label }),
      html({ autoCloseTags: !readOnly }),
    ],
    [label, readOnly]
  );

  return (
    <CodeMirror
      value={value}
      theme={dark ? darkTheme : lightTheme}
      extensions={extensions}
      basicSetup={basicSetup}
      readOnly={readOnly}
      onChange={onChange}
      height='100%'
      className='h-full'
    />
  );
};
