import js from '@eslint/js';
import tseslint from '@typescript-eslint/eslint-plugin';
import reactHooks from 'eslint-plugin-react-hooks';
import reactRefresh from 'eslint-plugin-react-refresh';
import { defineConfig } from 'eslint/config';
import globals from 'globals';

export default defineConfig([
  { ignores: ['dist/**', 'src/lib/unclutter-wasm/**', 'storybook-static/**'] },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs['flat/recommended'],
      reactHooks.configs.flat.recommended,
    ],
    languageOptions: {
      globals: globals.browser,
    },
    plugins: {
      'react-refresh': reactRefresh,
    },
    rules: {
      'react-refresh/only-export-components': 'warn',
    },
  },
  {
    files: ['**/*.stories.tsx', '.storybook/**'],
    rules: {
      'react-refresh/only-export-components': 'off',
    },
  },
]);
