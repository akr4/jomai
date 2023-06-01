import { styled } from '@stitches/react';
import * as TabsPrimitive from '@radix-ui/react-tabs';

const StyledList = styled(TabsPrimitive.List, {
  display: 'flex',
  borderBottom: `1px solid $separator`,
});

const StyledTrigger = styled(TabsPrimitive.Trigger, {
  backgroundColor: 'inherit',
  padding: '$medium',
  marginBottom: -1,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  fontSize: '$medium',
  lineHeight: 1,
  color: '$text1',
  userSelect: 'none',
  border: 'none',
  borderBottom: `1px solid $separator`,
  '&:hover': { color: '$highlighted' },
  '&[data-state="active"]': {
    color: '$primary',
    borderBottom: `1px solid $primary`,
  },
  '&[data-state="inactive"]': {
    cursor: 'pointer',
  },
});

export const Root = TabsPrimitive.Root;
export const List = StyledList;
export const Trigger = StyledTrigger;
export const Content = TabsPrimitive.Content;
