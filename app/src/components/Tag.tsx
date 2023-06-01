import React, { forwardRef } from 'react';
import { styled } from './stitches.config';
import { Button } from './Button';
import { Icon } from './Icon';
import { Text } from './Text';
import { Spacer } from './Spacer';

const Root = styled('div', {
  display: 'inline-flex',
  alignItems: 'center',
  backgroundColor: '$surface',
  fontSize: '$small',
  fontWeight: '500',
  color: '$text2',
  padding: '$x-small $small',
  borderRadius: 4,
});

const CloseButton = ({ onClick }: { onClick: () => void }) => (
  <Button variant="link" onClick={onClick}>
    <Icon iconName="circle-xmark" />
  </Button>
);

export const Tag = React.memo(
  forwardRef<
    HTMLDivElement,
    {
      children: React.ReactNode;
      onClick?: () => void;
      onClose?: () => void;
    }
  >(({ children, onClick, onClose }, ref) => {
    return (
      <Root ref={ref} onClick={onClick}>
        <Text>{children}</Text>

        {onClose != null && (
          <>
            <Spacer width="small" />
            <CloseButton onClick={onClose} />
          </>
        )}
      </Root>
    );
  }),
);
