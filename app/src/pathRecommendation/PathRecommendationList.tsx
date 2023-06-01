import React, { useCallback } from 'react';
import { PathRecommendation } from '../models/Watch';
import { Column } from '../components/Column';
import { Text } from '../components/Text';
import { Row } from '../components/Row';
import { Spacer } from '../components/Spacer';
import { Icon } from '../components/Icon';
import { useTranslation } from 'react-i18next';
import * as Checkbox from '../components/Checkbox';
import { Label } from '../components/Label';

export const PathRecommendationList: React.FC<{
  pathRecommendations: PathRecommendation[];
  selectedPathRecommendations: PathRecommendation[];
  onSelect: (pathRecommendation: PathRecommendation, isSelect: boolean) => void;
}> = ({ pathRecommendations, selectedPathRecommendations, onSelect }) => {
  return (
    <Column gap="medium">
      {pathRecommendations.map((pathRecommendation) => {
        if (pathRecommendation.type === 'documents') {
          return (
            <PathRecommendationRow
              id={pathRecommendation.type}
              key={pathRecommendation.type}
              checked={selectedPathRecommendations.some(
                (r) => r.type === pathRecommendation.type,
              )}
              onChange={(checked) => onSelect(pathRecommendation, checked)}
            >
              <DocumentsDir pathRecommendation={pathRecommendation} />
            </PathRecommendationRow>
          );
        } else if (pathRecommendation.type === 'obsidian') {
          return (
            <PathRecommendationRow
              id={pathRecommendation.type}
              key={pathRecommendation.type}
              checked={selectedPathRecommendations.some(
                (r) => r.type === pathRecommendation.type,
              )}
              onChange={(checked) => onSelect(pathRecommendation, checked)}
            >
              <ObsidianDir pathRecommendation={pathRecommendation} />
            </PathRecommendationRow>
          );
        }
      })}
    </Column>
  );
};

const PathRecommendationRow: React.FC<{
  id: string;
  children: React.ReactNode;
  checked: boolean;
  onChange: (checked: boolean) => void;
}> = ({ id, children, checked, onChange }) => {
  const handleChange = useCallback((checked: boolean | 'indeterminate') => {
    console.log(JSON.stringify({ checked }, null, 2));
    if (checked === 'indeterminate') {
      return;
    }
    onChange(checked);
  }, []);

  return (
    <Row>
      <Checkbox.Root checked={checked} onCheckedChange={handleChange} id={id}>
        <Checkbox.Indicator>
          {checked && <Icon iconName="check" />}
        </Checkbox.Indicator>
      </Checkbox.Root>

      <Label htmlFor={id}>
        <Row>
          <Spacer width="medium" />
          {children}
        </Row>
      </Label>
    </Row>
  );
};

const DocumentsDir: React.FC<{ pathRecommendation: PathRecommendation }> = ({
  pathRecommendation,
}) => {
  const { t } = useTranslation();
  return (
    <Column>
      <Text textStyle="body1">{t('pathRecommendations.paths.documents')}</Text>
      <Text textStyle="caption">{pathRecommendation.path}</Text>
    </Column>
  );
};

const ObsidianDir: React.FC<{ pathRecommendation: PathRecommendation }> = ({
  pathRecommendation,
}) => {
  const { t } = useTranslation();
  return (
    <Column>
      <Text textStyle="body1">{t('pathRecommendations.paths.obsidian')}</Text>
      <Text textStyle="caption">{pathRecommendation.path}</Text>
    </Column>
  );
};
