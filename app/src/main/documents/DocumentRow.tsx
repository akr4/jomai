import React, { forwardRef, MutableRefObject } from 'react';
import {
  Document,
  isSearchResultDocument,
  SearchResultDocument,
} from '../../models/Document';
import { Text } from '../../components/Text';
import { Column } from '../../components/Column';
import { Row } from '../../components/Row';
import { Spacer } from '../../components/Spacer';
import { Highlight } from './Highlight';
import { useTranslation } from 'react-i18next';
import { DocumentTitle } from './DocumentTitle';
import { DocumentMenuButton } from './DocumentMenuButton';
import { TagRow } from './TagRow';
import { Box } from '../../components/Box';

type Props = {
  document: Document | SearchResultDocument;
  isSelected: boolean;
  isMenuOpen: MutableRefObject<boolean>;
  onTagClick: (tag: string) => void;
  onClick: () => void;
};

export const DocumentRow = React.memo(
  forwardRef<HTMLDivElement, Props>(
    ({ document, isSelected, isMenuOpen, onTagClick, onClick }, ref) => {
      const { t } = useTranslation();

      return (
        <Box highlighted={isSelected} onClick={onClick}>
          <Row padding="medium">
            <Column ref={ref} align="start" gap="small">
              <Row align="center">
                <DocumentTitle document={document} />
              </Row>
              {isSearchResultDocument(document) &&
                document.highlight != null &&
                document.highlight.length > 0 && (
                  <Highlight value={document.highlight} />
                )}
              <Text textStyle="caption">{document.path}</Text>
              {document.tags?.length > 0 && (
                <TagRow tags={document.tags} onTagClick={onTagClick} />
              )}
              <Row gap="small">
                <Text textStyle="caption">
                  {t('documents.createdAt', {
                    timestamp: new Date(document.createdAt),
                  })}
                </Text>
                <Text textStyle="caption">
                  {t('documents.modifiedAt', {
                    timestamp: new Date(document.modifiedAt),
                  })}
                </Text>
              </Row>
            </Column>
            <Spacer stretch />
            <Box paddingTop="small">
              <DocumentMenuButton document={document} isMenuOpen={isMenuOpen} />
            </Box>
          </Row>
        </Box>
      );
    },
  ),
);

DocumentRow.displayName = 'DocumentRow';
