import React from 'react';
import { Html } from '../../components/Html';
import { styled } from '../../components/stitches.config';

const StyledHtml = styled(Html, {
  '& > b': {
    backgroundColor: 'rgba(53, 208, 74, 0.3)',
    padding: 1,
    margin: 2,
    borderRadius: 2,
  },
});

export const Highlight: React.FC<{ value: string }> = ({ value }) => {
  return <StyledHtml textStyle="body2" value={value} />;
};
