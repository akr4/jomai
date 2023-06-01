import { Text } from '../../components/Text';
import { Document, getTitle } from '../../models/Document';
import { Spacer } from '../../components/Spacer';
import { Icon } from '../../components/Icon';
import React, { useCallback } from 'react';
import { Row } from '../../components/Row';
import { useTranslation } from 'react-i18next';
import { Button } from '../../components/Button';
import { openFile } from './actions';
import * as Tooltip from '../../components/Tooltip';

type Props = { document: Document };

export const DocumentTitle: React.FC<Props> = React.memo(({ document }) => {
  const { t } = useTranslation();

  const handleClick = useCallback(async () => {
    await openFile(document.path);
  }, []);

  return (
    <Tooltip.Root>
      <Tooltip.Trigger asChild>
        <Button variant="link" onClick={handleClick}>
          <Row align="center">
            <Text textStyle="body1">{getTitle(document)}</Text>
            <Spacer width="small" />
            <Icon iconName="arrow-up-right-from-square" textStyle="body1" />
          </Row>
        </Button>
      </Tooltip.Trigger>
      <Tooltip.Content>{t('documents.openInDefaultApp')}</Tooltip.Content>
    </Tooltip.Root>
  );
});
DocumentTitle.displayName = 'DocumentTitle';
