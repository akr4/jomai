import { css } from '@stitches/react';

export const variantStyles = css({
  variants: {
    disabled: {
      true: {
        filter: 'brightness(75%)',
      },
    },
    highlighted: {
      true: {
        backgroundColor: '$highlightedBackground',
      },
    },
  },
});
