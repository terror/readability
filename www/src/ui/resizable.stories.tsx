import type { Meta, StoryObj } from '@storybook/react-vite';

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './resizable';

const Panes = ({ withHandle = false }: { withHandle?: boolean }) => (
  <div className='h-72 w-[560px] rounded-lg shadow-[0_0_0_1px_var(--line)]'>
    <ResizablePanelGroup orientation='horizontal'>
      <ResizablePanel defaultSize='40%' minSize='20%'>
        <div className='text-ink-3 flex h-full items-center justify-center text-sm'>
          Source
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle={withHandle} />
      <ResizablePanel defaultSize='60%' minSize='30%'>
        <div className='text-ink-3 flex h-full items-center justify-center text-sm'>
          Reader
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  </div>
);

const meta = {
  component: Panes,
  title: 'UI/Resizable',
} satisfies Meta<typeof Panes>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithHandle: Story = { args: { withHandle: true } };
