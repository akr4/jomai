import React from 'react';
import { styled } from './stitches.config';
import { spacingStyles } from './styles/spacingStyles';

export const Button = React.memo(
  styled('button', spacingStyles, {
    all: 'unset',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: 4,
    lineHeight: 1,
    cursor: 'pointer',
    '&:hover': { filter: 'brightness(120%)' },

    variants: {
      size: {
        small: {
          fontSize: '$small',
          padding: '$x-small $small',
        },
        medium: {
          fontSize: '$medium',
          padding: '$small $medium',
        },
      },
      variant: {
        primary: {
          backgroundColor: '$primaryBackground',
          color: '$text1',
          fontWeight: 500,
        },
        default: {
          backgroundColor: '$surface',
          color: '$text1',
        },
        link: {
          color: 'inherit',
          border: 'none',
          backgroundColor: 'inherit',
        },
        ghost: {
          border: 'none',
        },
      },
    },
  }),
);
