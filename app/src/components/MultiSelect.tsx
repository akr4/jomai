import { Row } from './Row';
import React, { forwardRef } from 'react';
import { Column } from './Column';
import { Button } from './Button';
import { Text } from './Text';
import { Box } from './Box';

type Item = {
  label: string;
};

type Props = {
  items: Item[];
  selectedIndex: number;
  onChange: (index: number) => void;
};

export const MultiSelect = React.memo(
  forwardRef<HTMLDivElement, Props>(
    ({ items, selectedIndex, onChange }, ref) => {
      return (
        <Row gap="small">
          {items.map((item, index) => {
            return (
              <Button
                key={index}
                variant="link"
                onClick={() => onChange(index)}
              >
                <Column>
                  <Text
                    css={{
                      color: index === selectedIndex ? '$text1' : '$text2',
                      fontSize: '$small',
                    }}
                  >
                    {item.label}
                  </Text>
                  <Box
                    css={{
                      marginTop: 4,
                      borderBottomWidth: '1px',
                      borderBottomStyle: 'solid',
                      borderBottomColor:
                        index === selectedIndex ? '$primary' : 'transparent',
                    }}
                  />
                </Column>
              </Button>
            );
          })}
        </Row>
      );
    },
  ),
);
