import { findIconDefinition, library } from '@fortawesome/fontawesome-svg-core';
import {
  faArrowUpRightFromSquare,
  faCheck,
  faCircleXmark,
  faClipboard,
  faEllipsisVertical,
  faFolder,
  faMagnifyingGlass,
  faPlus,
  faTrashCan,
  IconPrefix,
} from '@fortawesome/pro-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { forwardRef } from 'react';
import { styled } from './stitches.config';
import { textStyles } from './styles/textStyles';
import type * as Polymorphic from '@radix-ui/react-polymorphic';
import { sizeStyles } from './styles/sizeStyles';

library.add(faArrowUpRightFromSquare);
library.add(faMagnifyingGlass);
library.add(faPlus);
library.add(faCircleXmark);
library.add(faEllipsisVertical);
library.add(faFolder);
library.add(faTrashCan);
library.add(faClipboard);
library.add(faCheck);

export type IconName =
  | 'arrow-up-right-from-square'
  | 'magnifying-glass'
  | 'plus'
  | 'circle-xmark'
  | 'check'
  | 'ellipsis-vertical'
  | 'folder'
  | 'trash-can'
  | 'clipboard';

const PrefixMap: Record<IconName, IconPrefix> = {
  'arrow-up-right-from-square': 'far',
  'magnifying-glass': 'far',
  plus: 'far',
  'circle-xmark': 'far',
  'ellipsis-vertical': 'far',
  folder: 'far',
  'trash-can': 'far',
  clipboard: 'far',
  check: 'far',
};

const Div = styled('div', textStyles, sizeStyles);
type ComponentType = Polymorphic.ForwardRefComponent<
  'div',
  Partial<Polymorphic.OwnProps<typeof Div>> & { iconName: IconName }
>;

export const Icon = React.memo(
  forwardRef(({ iconName, ...props }, ref) => {
    const prefix = PrefixMap[iconName];
    const icon = findIconDefinition({ prefix, iconName });
    return (
      <Div ref={ref} {...props}>
        <FontAwesomeIcon icon={icon} />
      </Div>
    );
  }),
) as ComponentType;
