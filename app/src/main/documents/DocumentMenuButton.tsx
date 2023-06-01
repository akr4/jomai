import React, { MutableRefObject, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { Document } from '../../models/Document';
import { copyToClipboard, openContainingFolder, openFile } from './actions';
import * as DropdownMenu from '../../components/DropdownMenu';
import { IconButton } from '../../components/IconButton';

export const DocumentMenuButton: React.FC<{
  document: Document;
  isMenuOpen: MutableRefObject<boolean>;
}> = React.memo(({ document, isMenuOpen }) => {
  const { t } = useTranslation();

  const handleOpenClick = useCallback(async (e: Event) => {
    e.stopPropagation();
    await openFile(document.path);
  }, []);

  const handleOpenContainingFolderClick = useCallback(
    async (e: Event) => {
      e.stopPropagation();
      await openContainingFolder(document.path);
    },
    [document.path],
  );

  const handleCopyPathToClipboardClick = useCallback(
    async (e: Event) => {
      e.stopPropagation();
      await copyToClipboard(document.path);
    },
    [document.path],
  );

  const handleOpenChange = useCallback((open: boolean) => {
    if (isMenuOpen.current != null) {
      isMenuOpen.current = open;
    }
  }, []);

  return (
    <DropdownMenu.Root onOpenChange={handleOpenChange}>
      <DropdownMenu.Trigger asChild>
        <IconButton iconName="ellipsis-vertical" color="text2" />
      </DropdownMenu.Trigger>

      <DropdownMenu.Content>
        <DropdownMenu.BasicMenuItem
          iconName="arrow-up-right-from-square"
          label={t('documents.menu.openInDefaultApp')}
          command="︎↩︎"
          onSelect={handleOpenClick}
        />
        <DropdownMenu.BasicMenuItem
          iconName="folder"
          label={t('documents.menu.openContainingFolder')}
          command="︎F"
          onSelect={handleOpenContainingFolderClick}
        />
        <DropdownMenu.BasicMenuItem
          iconName="clipboard"
          label={t('documents.menu.copyPathToClipboard')}
          command="︎C︎"
          onSelect={handleCopyPathToClipboardClick}
        />
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
});
DocumentMenuButton.displayName = 'DocumentMenuButton';
