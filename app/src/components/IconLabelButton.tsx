import { Button } from './Button';
import { Row } from './Row';
import { Icon, IconName } from './Icon';
import { Spacer } from './Spacer';
import { Text } from './Text';
import React from 'react';
import * as Stitches from '@stitches/react';

type Props = {
  onClick: (event: React.MouseEvent<HTMLButtonElement>) => void;
  text: string;
  iconName: IconName;
} & Stitches.ComponentProps<typeof Button>;

export const IconLabelButton = React.memo(
  ({ onClick, text, iconName, ...props }: Props) => (
    <Button variant="default" size="small" onClick={onClick} {...props}>
      <Row padding="small" align="center">
        <Icon iconName={iconName} />
        <Spacer width="medium" />
        <Text>{text}</Text>
      </Row>
    </Button>
  ),
);
IconLabelButton.displayName = 'IconLabelButton';
