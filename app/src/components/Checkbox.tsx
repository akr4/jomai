import { styled } from '@stitches/react';
import * as CheckboxPrimitive from '@radix-ui/react-checkbox';

const StyledCheckbox = styled(CheckboxPrimitive.Root, {
  all: 'unset',
  backgroundColor: '$background',
  width: 25,
  height: 25,
  borderRadius: 4,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  border: '1px solid $border',
  '&:hover': { backgroundColor: '$highlightedBackground' },
});

const StyledIndicator = styled(CheckboxPrimitive.Indicator, {
  color: '$primary',
});

// Exports
export const Root = StyledCheckbox;
export const Indicator = StyledIndicator;
