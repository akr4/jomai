import React from 'react';
import { Text } from '../../components/Text';
import { useTranslation } from 'react-i18next';

export const Explanation: React.FC = React.memo(() => {
  const { t } = useTranslation();
  return (
    <Text css={{ flexShrink: 1 }} textStyle="caption">
      {t('watches.explanation')}
    </Text>
  );
});
Explanation.displayName = 'Explanation';
