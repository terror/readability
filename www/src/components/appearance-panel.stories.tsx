import { Appearance, defaultAppearance } from '@/lib/appearance';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { useState } from 'react';

import { AppearancePanel } from './appearance-panel';

const Panel = () => {
  const [appearance, setAppearance] = useState<Appearance>(defaultAppearance);

  return (
    <div className='w-72'>
      <AppearancePanel
        appearance={appearance}
        updateAppearance={(update) =>
          setAppearance((previous) => ({ ...previous, ...update }))
        }
      />
    </div>
  );
};

const meta = {
  component: Panel,
  title: 'App/Appearance panel',
} satisfies Meta<typeof Panel>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
