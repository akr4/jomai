import { Column } from '../../components/Column';
import { Spacer } from '../../components/Spacer';
import { Row } from '../../components/Row';
import React from 'react';
import { motion } from 'framer-motion';
import { styled } from '../../components/stitches.config';
import { keyframes } from '@stitches/react';

const rotate = keyframes({
  '0%': { transform: 'translateX(-100%)' },
  '100%': { transform: 'translateX(100%)' },
});

const Base = styled(motion.div, {
  background: '$surface',
  position: 'relative',
  overflow: 'hidden',
  '&::before': {
    content: '',
    display: 'block',
    width: '100%',
    height: '100%',
    background: `linear-gradient(90deg, transparent, $highlightedBackground, transparent)`,
    position: 'absolute',
    top: 0,
    left: 0,
    '@media (prefers-reduced-motion: no-preference)': {
      animation: `${rotate} 1.2s linear 0`,
      animationIterationCount: 'infinite',
    },
  },
});

const Rectangle = ({
  width,
  height,
}: {
  width: string | number;
  height: string | number;
}) => (
  <Base
    css={{
      width,
      height,
    }}
  />
);

export const SearchResultsSkeleton: React.FC = React.memo(() => {
  return (
    <Column
      gap="large"
      padding="medium"
      css={{
        cursor: 'progress',
      }}
    >
      <Rectangle width={150} height={18} />
      {[...Array(10).keys()].map((i) => (
        <Column key={i}>
          <Rectangle key={`${i}-1`} width="200px" height="22px" />
          <Spacer height="small" />
          <Rectangle key={`${i}-2`} width="600px" height="14px" />
          <Spacer height="small" />
          <Row gap="small">
            {[...Array(2).keys()].map((j) => (
              <Rectangle key={`${i}-${j}`} width="60px" height="18px" />
            ))}
          </Row>
          <Spacer height="small" />
          <Rectangle key={`${i}-3`} width="400px" height="18px" />
        </Column>
      ))}
    </Column>
  );
});
SearchResultsSkeleton.displayName = 'SearchResultsSkeleton';
