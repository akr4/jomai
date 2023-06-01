import React from 'react';
import { Text } from '../components/Text';
import { Column } from '../components/Column';
import { Spacer } from '../components/Spacer';
import { useTranslation } from 'react-i18next';

export const Header: React.FC = () => {
  const { t } = useTranslation();
  return (
    <Column>
      <Text textStyle="h1">{t('pathRecommendations.header.title')}</Text>
      <Spacer height="small" />
      <Text textStyle="body1">{t('pathRecommendations.header.text1')}</Text>
      <Text textStyle="body1">{t('pathRecommendations.header.text2')}</Text>
    </Column>
  );
};
