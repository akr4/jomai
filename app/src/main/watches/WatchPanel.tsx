import React from 'react';
import { Column } from '../../components/Column';
import { WatchRow } from './WatchRow';
import { useWatchState } from '../../helpers/useWatchState';
import { WatchJobReport } from '../../models/Watch';
import { Row } from '../../components/Row';
import { useTranslation } from 'react-i18next';
import { AddWatchButton } from './AddWatchButton';
import { Explanation } from './Explanation';
import { Spacer } from '../../components/Spacer';

export const WatchPanel: React.FC = () => {
  const { t } = useTranslation();
  const watchState = useWatchState();

  return (
    <Column
      css={{
        height: 'calc(100vh - 40px)',
      }}
    >
      <Row padding="large" justify="start">
        <Explanation />
        <Spacer width="medium" stretch />
        <AddWatchButton />
      </Row>
      <Column
        css={{
          overflowY: 'scroll',
        }}
      >
        {watchState.watches.map((watch) => {
          return (
            <WatchRow
              key={watch.path}
              watch={watch}
              jobReport={findJobReportForWatch(watch.id, watchState.jobReports)}
            />
          );
        })}
      </Column>
    </Column>
  );
};

const findJobReportForWatch = (
  watchId: number,
  jobReports: WatchJobReport[],
) => {
  return jobReports.find((jobReport) => jobReport.watch.id === watchId);
};
