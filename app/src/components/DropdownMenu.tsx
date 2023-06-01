import React from 'react';
import * as Stitches from '@stitches/react';
import { keyframes } from '@stitches/react';
import * as DropdownMenuPrimitive from '@radix-ui/react-dropdown-menu';
import { Icon, IconName } from './Icon';
import { Spacer } from './Spacer';
import { styled } from './stitches.config';

const slideUpAndFade = keyframes({
  '0%': { opacity: 0, transform: 'translateY(2px)' },
  '100%': { opacity: 1, transform: 'translateY(0)' },
});

const slideRightAndFade = keyframes({
  '0%': { opacity: 0, transform: 'translateX(-2px)' },
  '100%': { opacity: 1, transform: 'translateX(0)' },
});

const slideDownAndFade = keyframes({
  '0%': { opacity: 0, transform: 'translateY(-2px)' },
  '100%': { opacity: 1, transform: 'translateY(0)' },
});

const slideLeftAndFade = keyframes({
  '0%': { opacity: 0, transform: 'translateX(2px)' },
  '100%': { opacity: 1, transform: 'translateX(0)' },
});

const contentStyles = {
  minWidth: 220,
  backgroundColor: '$surface',
  borderRadius: 6,
  padding: 5,
  boxShadow:
    '0px 10px 38px -10px rgba(22, 23, 24, 0.35), 0px 10px 20px -15px rgba(22, 23, 24, 0.2)',
  '@media (prefers-reduced-motion: no-preference)': {
    animationDuration: '400ms',
    animationTimingFunction: 'cubic-bezier(0.16, 1, 0.3, 1)',
    willChange: 'transform, opacity',
    '&[data-state="open"]': {
      '&[data-side="top"]': { animationName: slideDownAndFade },
      '&[data-side="right"]': { animationName: slideLeftAndFade },
      '&[data-side="bottom"]': { animationName: slideUpAndFade },
      '&[data-side="left"]': { animationName: slideRightAndFade },
    },
  },
};

const StyledContent = styled(DropdownMenuPrimitive.Content, {
  ...contentStyles,
});

const StyledArrow = styled(DropdownMenuPrimitive.Arrow, {
  fill: '$surface',
});

const Content = ({
  children,
  ...props
}: {
  children: React.ReactNode;
} & Stitches.ComponentProps<typeof StyledContent>) => (
  <DropdownMenuPrimitive.Portal>
    <StyledContent collisionPadding={16} {...props}>
      {children}
      <StyledArrow />
    </StyledContent>
  </DropdownMenuPrimitive.Portal>
);

const itemStyles = {
  all: 'unset',
  fontSize: '$small',
  lineHeight: 1,
  color: '$text2',
  borderRadius: 3,
  display: 'flex',
  alignItems: 'center',
  padding: '$small',
  position: 'relative',
  userSelect: 'none',

  '&[data-disabled]': {
    color: '$text3',
    pointerEvents: 'none',
  },

  '&[data-highlighted]': {
    backgroundColor: '$highlightedBackground',
    color: '$text1',
  },
};

const StyledItem = styled(DropdownMenuPrimitive.Item, itemStyles);

const RightSlot = styled('div', {
  marginLeft: 'auto',
  paddingLeft: 20,
  '[data-highlighted] > &': { color: '$text1' },
  '[data-disabled] &': { color: '$text3' },
});

const BasicMenuItem = ({
  iconName,
  label,
  command,
  disabled,
  onSelect,
}: {
  iconName: IconName;
  label: string;
  command?: string;
  disabled?: boolean;
  onSelect: (e: Event) => void;
}) => (
  <StyledItem onSelect={onSelect} disabled={disabled}>
    <Icon
      iconName={iconName}
      css={{
        width: 18,
        textAlign: 'right',
      }}
    />
    <Spacer width="small" />
    {label}
    {command != null && <RightSlot>{command}︎</RightSlot>}
  </StyledItem>
);

const Root = DropdownMenuPrimitive.Root;
const Trigger = DropdownMenuPrimitive.Trigger;
const Item = StyledItem;

// Exports
export { Root, Trigger, Content, Item, RightSlot, BasicMenuItem };
