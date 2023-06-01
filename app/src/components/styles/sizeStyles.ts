import { css } from '@stitches/react';

export const sizeStyles = css({
  variants: {
    width: {
      'x-small': {
        width: '$x-small',
      },
      small: {
        width: '$small',
      },
      medium: {
        width: '$medium',
      },
      large: {
        width: '$large',
      },
    },

    height: {
      'x-small': {
        height: '$x-small',
      },
      small: {
        height: '$small',
      },
      medium: {
        height: '$medium',
      },
      large: {
        height: '$large',
      },
    },
  },
});
