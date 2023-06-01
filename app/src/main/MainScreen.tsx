import React from 'react';
import { BasicLayout } from '../layout/BasicLayout';
import * as Tabs from '../components/Tabs';
import { DocumentPanel } from './documents/DocumentPanel';
import { WatchPanel } from './watches/WatchPanel';
import { useTranslation } from 'react-i18next';

export const MainScreen: React.FC = () => {
  const { t } = useTranslation();

  return (
    <BasicLayout>
      <Tabs.Root defaultValue="documents">
        <Tabs.List>
          <Tabs.Trigger value="documents">{t('tab.documents')}</Tabs.Trigger>
          <Tabs.Trigger value="watches">{t('tab.watches')}</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="documents">
          <DocumentPanel />
        </Tabs.Content>
        <Tabs.Content value="watches">
          <WatchPanel />
        </Tabs.Content>
      </Tabs.Root>
    </BasicLayout>
  );
};
