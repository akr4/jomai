import React, { useCallback } from 'react';
import { Text } from '../../components/Text';
import { Row } from '../../components/Row';
import { ALL_SORTS, Sort } from '../../models/Document';
import { Spacer } from '../../components/Spacer';
import { MultiSelect } from '../../components/MultiSelect';
import { useTranslation } from 'react-i18next';

export const SearchResultsStats: React.FC<{
  count: number;
  showSort: boolean;
  sort: Sort;
  onSortChange: (sort: Sort) => void;
}> = ({ count, showSort, sort, onSortChange }) => {
  const { t } = useTranslation();
  const selectedIndex = ALL_SORTS.indexOf(sort);

  const handleSortChange = useCallback((index: number) => {
    onSortChange(ALL_SORTS[index]);
  }, []);
  return (
    <Row padding="medium">
      <Text textStyle="caption">{t('documents.found', { count })}</Text>
      <Spacer stretch />
      {showSort && count > 0 && (
        <MultiSelect
          items={[
            { label: t('documents.sort.relative') },
            { label: t('documents.sort.timestamp') },
          ]}
          selectedIndex={selectedIndex}
          onChange={handleSortChange}
        />
      )}
    </Row>
  );
};
