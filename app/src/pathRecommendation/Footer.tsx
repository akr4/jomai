import React from 'react';
import { Row } from '../components/Row';
import { Button } from '../components/Button';
import { useTranslation } from 'react-i18next';

export const Footer: React.FC<{ onNextClick: () => void }> = ({
  onNextClick,
}) => {
  const { t } = useTranslation();
  return (
    <Row justify="end">
      <Button variant="primary" onClick={onNextClick} size="medium">
        {t('pathRecommendations.footer.button')}
      </Button>
    </Row>
  );
};
