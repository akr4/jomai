import React, { ReactNode } from 'react';
import { keyframes, styled } from '@stitches/react';
import { blackA } from '@radix-ui/colors';
import * as AlertDialogPrimitive from '@radix-ui/react-alert-dialog';

const overlayShow = keyframes({
  '0%': { opacity: 0 },
  '100%': { opacity: 1 },
});

const contentShow = keyframes({
  '0%': { opacity: 0, transform: 'translate(-50%, -48%) scale(.96)' },
  '100%': { opacity: 1, transform: 'translate(-50%, -50%) scale(1)' },
});

const StyledOverlay = styled(AlertDialogPrimitive.Overlay, {
  backgroundColor: blackA.blackA9,
  position: 'fixed',
  inset: 0,
  '@media (prefers-reduced-motion: no-preference)': {
    animation: `${overlayShow} 150ms cubic-bezier(0.16, 1, 0.3, 1)`,
  },
});

const StyledContent = styled(AlertDialogPrimitive.Content, {
  backgroundColor: '$background',
  borderRadius: 6,
  boxShadow:
    'hsl(206 22% 7% / 35%) 0px 10px 38px -10px, hsl(206 22% 7% / 20%) 0px 10px 20px -15px',
  position: 'fixed',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: '90vw',
  maxWidth: '500px',
  maxHeight: '85vh',
  padding: 25,
  '@media (prefers-reduced-motion: no-preference)': {
    animation: `${contentShow} 150ms cubic-bezier(0.16, 1, 0.3, 1)`,
  },
  '&:focus': { outline: 'none' },
});

function Content({
  children,
  ...props
}: { children: ReactNode } & React.ComponentProps<typeof StyledContent>) {
  return (
    <AlertDialogPrimitive.Portal>
      <StyledOverlay />
      <StyledContent {...props}>{children}</StyledContent>
    </AlertDialogPrimitive.Portal>
  );
}

const StyledTitle = styled(AlertDialogPrimitive.Title, {
  margin: 0,
  color: '$text1',
  fontSize: '$large',
  fontWeight: 500,
});

const StyledDescription = styled(AlertDialogPrimitive.Description, {
  marginBottom: 20,
  color: '$text2',
  fontSize: '$medium',
  lineHeight: 1.5,
});

export { Content };
export const Root = AlertDialogPrimitive.Root;
export const Trigger = AlertDialogPrimitive.Trigger;
export const Title = StyledTitle;
export const Description = StyledDescription;
export const Action = AlertDialogPrimitive.Action;
export const Cancel = AlertDialogPrimitive.Cancel;
