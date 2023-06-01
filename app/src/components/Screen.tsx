import React from 'react';
import { styled } from './stitches.config';
import { Box } from './Box';

export const Screen = React.memo(
  styled(Box, {
    height: '100vh',
    overflowY: 'hidden',
    overScrollBehavior: 'none',
    userSelect: 'none',
    backgroundColor: '$background',
  }),
);
