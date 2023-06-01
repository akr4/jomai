import React, { useCallback } from 'react';
import { Column } from '../../components/Column';
import { Text } from '../../components/Text';
import { WatchFull, WatchJobReport } from '../../models/Watch';
import { Row } from '../../components/Row';
import { Spacer } from '../../components/Spacer';
import { useDeleteWatch } from '../../helpers/useDeleteWatch';
import { useTranslation } from 'react-i18next';
import { WatchMenuButton } from './WatchMenuButton';
import { JobReportView } from './JobReportView';
import { Box } from '../../components/Box';

export const WatchRow: React.FC<{
  watch: WatchFull;
  jobReport?: WatchJobReport;
}> = ({ watch, jobReport }) => {
  const { t } = useTranslation();
  const deleteWatch = useDeleteWatch();

  const handleDeleteWatch = useCallback(async () => {
    deleteWatch(watch.path);
  }, []);

  const canDelete = watch.status === 'active' || watch.status === 'adding';
  const disabled = watch.status !== 'active';

  return (
    <Row padding="medium">
      <Column gap="small">
        <Text textStyle="body1" disabled={disabled}>
          {watch.path}
        </Text>
        <Row>
          <Text textStyle="caption">
            {t('watches.documentCount', { count: watch.documentCount })}
          </Text>
        </Row>
        <Text textStyle="caption" disabled={disabled}>
          {t('watches.createdAt', {
            timestamp: new Date(watch.createdAt),
          })}
        </Text>
        {jobReport != null && jobReport.status === 'running' && (
          <JobReportView
            watch={watch}
            jobReport={jobReport}
            disabled={disabled}
          />
        )}
      </Column>
      <Spacer stretch />
      <Box paddingTop="small">
        <WatchMenuButton canDelete={canDelete} onDelete={handleDeleteWatch} />
      </Box>
    </Row>
  );
};
