import { defaultAppearance } from '@/lib/appearance';
import type { Meta, StoryObj } from '@storybook/react-vite';

import { article } from '../stories/fixtures';
import { Reader } from './reader';

const meta = {
  args: {
    appearance: defaultAppearance,
    article,
    duration: 3.2,
    readable: true,
    url: 'https://harborledger.example/news/riverside-tram-line',
  },
  component: Reader,
  parameters: { layout: 'fullscreen' },
  title: 'App/Reader',
} satisfies Meta<typeof Reader>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Readable: Story = {};

export const NotReadable: Story = { args: { readable: false } };

export const SansWide: Story = {
  args: {
    appearance: {
      ...defaultAppearance,
      textSize: 18,
      typeface: 'sans',
      width: 'wide',
    },
  },
};
