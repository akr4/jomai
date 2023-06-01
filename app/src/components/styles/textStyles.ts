import { css } from '@stitches/react';

export const textStyles = css({
  variants: {
    textStyle: {
      h1: {
        color: '$text1',
        fontSize: '$large',
        fontWeight: 'bold',
      },
      body1: {
        color: '$text1',
        fontSize: '$medium',
      },
      body2: {
        color: '$text2',
        fontSize: '$small',
      },
      caption: {
        color: '$text3',
        fontSize: '$small',
      },
    },
  },
});
