import React from 'react';
import { styled } from './stitches.config';

export const Spacer = React.memo(
  styled('span', {
    display: 'inline-block',

    variants: {
      width: {
        'x-small': {
          width: 4,
        },
        small: {
          width: 8,
        },
        medium: {
          width: 16,
        },
        large: {
          width: 24,
        },
      },

      height: {
        'x-small': {
          height: 4,
        },
        small: {
          height: 8,
        },
        medium: {
          height: 16,
        },
        large: {
          height: 24,
        },
      },

      stretch: {
        true: {
          flexGrow: 1,
        },
      },
    },
  }),
);
