import { Button } from './Button';
import { Icon, IconName } from './Icon';
import React, { forwardRef } from 'react';
import * as Stitches from '@stitches/react';
import { config } from './stitches.config';
import { useTheme } from './ThemeProvider';

type Props = {
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;
  iconName: IconName;
  color: keyof typeof config.theme.colors;
} & Stitches.VariantProps<typeof Button>;

export const IconButton = React.memo(
  forwardRef<HTMLButtonElement, Props>(
    ({ onClick, iconName, color, ...props }: Props, ref) => {
      const theme = useTheme();
      return (
        <Button
          ref={ref}
          variant="ghost"
          onClick={onClick}
          padding="medium"
          css={{
            color: theme.colors[color],
          }}
          {...props}
        >
          <Icon iconName={iconName} />
        </Button>
      );
    },
  ),
);
IconButton.displayName = 'IconButton';
