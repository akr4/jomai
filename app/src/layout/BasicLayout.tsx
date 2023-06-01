import React from 'react';
import { Screen } from '../components/Screen';

export const BasicLayout: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  return <Screen>{children}</Screen>;
};
