import React from 'react';
import { styled } from './stitches.config';
import * as SeparatorPrimitive from '@radix-ui/react-separator';

export const Divider = React.memo(
  styled(SeparatorPrimitive.Root, {
    backgroundColor: '$separator',
    '&[data-orientation=horizontal]': { height: 1, width: '100%' },
    '&[data-orientation=vertical]': { height: '100%', width: 1 },
  }),
);
