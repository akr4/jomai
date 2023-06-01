import React from 'react';
import { styled } from './stitches.config';
import { sizeStyles } from './styles/sizeStyles';

export const TextField = React.memo(
  styled('input', sizeStyles, {
    appearance: 'none',
    color: '$text1',
    backgroundColor: '$background',
    fontSize: '$medium',
    border: '1px solid $sand10',
    padding: '$small',
    borderRadius: '4px',
    outline: 'none',
    '&:focus': {
      webkitOutlineColor: '$primary',
      border: '1px solid $primary',
      transition: '0.25s',
    },
  }),
);
