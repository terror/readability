import type { Meta, StoryObj } from '@storybook/react-vite';

import { Row, Section } from './section';
import { Switch } from './switch';

const Settings = () => (
  <div className='w-72'>
    <Section title='Parser options'>
      <div className='flex flex-col'>
        <Row label='Read JSON-LD metadata'>
          <Switch defaultChecked />
        </Row>
        <Row label='Keep all classes'>
          <Switch />
        </Row>
      </div>
    </Section>
  </div>
);

const meta = {
  component: Settings,
  title: 'UI/Section',
} satisfies Meta<typeof Settings>;

export default meta;

export const Default: StoryObj<typeof meta> = {};
