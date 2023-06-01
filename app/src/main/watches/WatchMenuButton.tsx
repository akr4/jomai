import React from 'react';
import { useTranslation } from 'react-i18next';
import * as DropdownMenu from '../../components/DropdownMenu';
import { IconButton } from '../../components/IconButton';

export const WatchMenuButton: React.FC<{
  canDelete: boolean;
  onDelete: () => void;
}> = React.memo(({ canDelete, onDelete }) => {
  const { t } = useTranslation();

  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <IconButton iconName="ellipsis-vertical" color="text2" />
      </DropdownMenu.Trigger>

      <DropdownMenu.Content sideOffset={5}>
        <DropdownMenu.BasicMenuItem
          iconName="trash-can"
          label={t('watches.menu.delete')}
          onSelect={onDelete}
          disabled={!canDelete}
        />
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
});
WatchMenuButton.displayName = 'WatchMenuButton';
