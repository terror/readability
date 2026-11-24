import '@fontsource-variable/inter';
import '@fontsource-variable/jetbrains-mono';
import '@fontsource-variable/literata';
import '@fontsource-variable/literata/wght-italic.css';
import type { Preview } from '@storybook/react-vite';

import '../src/index.css';

const preview: Preview = {
  decorators: [
    (Story, { globals }) => {
      document.documentElement.dataset.theme = globals.theme;
      return <Story />;
    },
  ],
  globalTypes: {
    theme: {
      description: 'Theme',
      toolbar: {
        dynamicTitle: true,
        items: ['light', 'sepia', 'dark'],
        title: 'Theme',
      },
    },
  },
  initialGlobals: {
    theme: 'light',
  },
  parameters: {
    layout: 'centered',
  },
};

export default preview;
