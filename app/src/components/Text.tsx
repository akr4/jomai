import React from 'react';
import { styled } from './stitches.config';
import { variantStyles } from './styles/variantStyles';
import { textStyles } from './styles/textStyles';

export const Text = React.memo(styled('span', textStyles, variantStyles));
