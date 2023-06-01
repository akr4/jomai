import React, { forwardRef } from 'react';

import { styled } from './stitches.config';
import { textStyles } from './styles/textStyles';
import { variantStyles } from './styles/variantStyles';
import type * as Polymorphic from '@radix-ui/react-polymorphic';

const P = React.memo(styled('p', textStyles, variantStyles));
type ComponentType = Polymorphic.ForwardRefComponent<
  typeof P,
  { value: string }
>;

export const Html = forwardRef<HTMLParagraphElement, { value: string }>(
  ({ value, ...props }, ref) => (
    <P ref={ref} dangerouslySetInnerHTML={{ __html: value }} {...props} />
  ),
) as ComponentType;
Html.displayName = 'Html';
