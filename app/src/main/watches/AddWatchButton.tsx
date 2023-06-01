import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useAddWatch } from '../../helpers/useAddWatch';
import { useTranslation } from 'react-i18next';
import * as Toast from '../../components/Toast';
import { IconLabelButton } from '../../components/IconLabelButton';
import { open as openDialog } from '@tauri-apps/api/dialog';
import { styled } from '../../components/stitches.config';

const Div = styled('div');

export const AddWatchButton: React.FC = React.memo(() => {
  const { t } = useTranslation();
  const { addWatch, error } = useAddWatch();
  const [open, setOpen] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout>>();

  const handleAddPath = useCallback(async () => {
    const selected = await openDialog({
      multiple: true,
      directory: true,
    });

    if (selected != null) {
      const paths = Array.isArray(selected) ? selected : [selected];
      addWatch(paths);
    }
  }, []);

  useEffect(() => {
    return () => {
      clearTimeout(timerRef.current);
    };
  });

  useEffect(() => {
    if (error != null) {
      setOpen(true);
      timerRef.current = setTimeout(() => {
        setOpen(false);
      }, 3000);
    }
  }, [error]);

  return (
    <Div css={{ flexGrow: 1, flexShrink: 0 }}>
      <IconLabelButton
        onClick={handleAddPath}
        iconName="plus"
        text={t('watches.addPath')}
      />
      {error != null && (
        <Toast.Root open={open} onOpenChange={setOpen}>
          <Toast.Title>{t('watches.errors.title')}</Toast.Title>
          <Toast.Description>
            {t(`watches.errors.${error.type}`)}
          </Toast.Description>
        </Toast.Root>
      )}
    </Div>
  );
});
AddWatchButton.displayName = 'AddWatchButton';
