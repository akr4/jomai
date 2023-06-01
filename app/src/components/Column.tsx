import React from 'react';
import { styled } from './stitches.config';
import { flexStyles } from './styles/flexStyles';
import { sizeStyles } from './styles/sizeStyles';
import { spacingStyles } from './styles/spacingStyles';

export const Column = React.memo(
  styled('div', flexStyles, sizeStyles, spacingStyles, {
    display: 'flex',
    flexDirection: 'column',
  }),
);
