import React from 'react';
import { Row } from '../../components/Row';
import { Text } from '../../components/Text';
import { Spacer } from '../../components/Spacer';
import { ProgressBar } from '../../components/ProgressBar';
import { Watch, WatchJobReport } from '../../models/Watch';
import { useTranslation } from 'react-i18next';

export const JobReportView: React.FC<{
  watch: Watch;
  jobReport: WatchJobReport;
  disabled: boolean;
}> = ({ watch, disabled, jobReport }) => {
  const { t } = useTranslation();
  return (
    <Row align="center">
      <Text textStyle="caption" disabled={disabled}>
        {t(`watches.job.status.${watch.status}`)}
      </Text>
      <Spacer width="medium" />
      <ProgressBar
        value={jobReport.progress.done}
        max={jobReport.progress.total}
      />
      <Spacer width="medium" />
      <Text
        textStyle="caption"
        disabled={disabled}
      >{`${Intl.NumberFormat().format(
        jobReport.progress.done,
      )} / ${Intl.NumberFormat().format(jobReport.progress.total)}`}</Text>
    </Row>
  );
};
