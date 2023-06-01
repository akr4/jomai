import { css } from '@stitches/react';

export const flexStyles = css({
  display: 'flex',

  variants: {
    align: {
      start: {
        alignItems: 'flex-start',
      },
      center: {
        alignItems: 'center',
      },
      end: {
        alignItems: 'flex-end',
      },
      stretch: {
        alignItems: 'stretch',
      },
      baseline: {
        alignItems: 'baseline',
      },
    },
    justify: {
      start: {
        justifyContent: 'flex-start',
      },
      center: {
        justifyContent: 'center',
      },
      end: {
        justifyContent: 'flex-end',
      },
      between: {
        justifyContent: 'space-between',
      },
      around: {
        justifyContent: 'space-around',
      },
    },
    gap: {
      small: {
        gap: '$small',
      },
      medium: {
        gap: '$medium',
      },
      large: {
        gap: '$large',
      },
    },
  },
});
