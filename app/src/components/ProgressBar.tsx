import React from 'react';
import { styled } from '@stitches/react';
import * as ProgressPrimitive from '@radix-ui/react-progress';

const StyledProgress = styled(ProgressPrimitive.Root, {
  position: 'relative',
  overflow: 'hidden',
  background: '$surface',
  borderRadius: 4,
  width: 200,
  height: 16,

  // Fix overflow clipping in Safari
  // https://gist.github.com/domske/b66047671c780a238b51c51ffde8d3a0
  transform: 'translateZ(0)',
});

const StyledIndicator = styled(ProgressPrimitive.Indicator, {
  backgroundColor: '$secondary',
  width: '100%',
  height: '100%',
});

export const ProgressBar: React.FC<{
  value: number;
  max: number;
}> = ({ value, max }) => {
  const progress = (value / max) * 100;
  return (
    <StyledProgress value={value} max={max}>
      <StyledIndicator
        style={{ transform: `translateX(-${100 - progress}%)` }}
      />
    </StyledProgress>
  );
};
