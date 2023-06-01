import {
  gray,
  grayDark,
  red,
  redDark,
  sand,
  sandDark,
  sky,
  skyDark,
  slate,
  slateDark,
} from '@radix-ui/colors';

// Spread the scales in your light and dark themes
import { createStitches, globalCss } from '@stitches/react';

const globalStyles = globalCss({
  body: {
    fontFamily:
      '"Helvetica Neue", Arial, "Hiragino Kaku Gothic ProN", "Hiragino Sans", Meiryo, sans-serif',
  },
});

const {
  styled,
  createTheme,
  theme: lightTheme,
  config,
} = createStitches({
  theme: {
    colors: {
      ...gray,
      ...red,
      ...slate,
      ...sand,
      ...sky,
      primary: '$sky11',
      primaryBackground: '$sky7',
      secondary: '$sky7',
      background: 'white',
      surface: '$slate5',
      border: '$slate7',
      separator: '$border',
      text1: '$sand12',
      text2: '$sand11',
      text3: '$sand9',
      errorBackground: '$red11',
      highlighted: 'black',
      highlightedBackground: '$slate3',
    },
    space: {
      'x-small': '4px',
      small: '8px',
      medium: '16px',
      large: '24px',
    },
    sizes: {
      small: '200px',
      medium: '400px',
      large: '600px',
    },
    fontSizes: {
      small: '16px',
      medium: '18px',
      large: '20px',
    },
  },
});

const darkTheme = createTheme('dark', {
  colors: {
    ...grayDark,
    ...redDark,
    ...slateDark,
    ...sandDark,
    ...skyDark,
    primary: '$sky11',
    primaryBackground: '$sky8',
    secondary: '$sky9',
    background: '$slate4',
    surface: '$slate7',
    border: '$slate7',
    separator: '$border',
    text1: '$sand12',
    text2: '$sand11',
    text3: '$sand9',
    errorBackground: '$red11',
    highlighted: 'white',
    highlightedBackground: '$slate6',
  },
});

export { globalStyles, styled, lightTheme, darkTheme, config };
