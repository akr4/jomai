import React from 'react';
import { styled } from './stitches.config';
import { sizeStyles } from './styles/sizeStyles';
import { variantStyles } from './styles/variantStyles';
import { spacingStyles } from './styles/spacingStyles';

export const Box = React.memo(
  styled('div', sizeStyles, spacingStyles, variantStyles),
);
