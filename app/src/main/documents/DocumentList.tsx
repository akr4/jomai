import React, { forwardRef, MutableRefObject, Ref } from 'react';
import { DocumentRow } from './DocumentRow';
import { Document, SearchResultDocument } from '../../models/Document';
import { Column } from '../../components/Column';
import { Virtuoso, VirtuosoHandle } from 'react-virtuoso';

type Props = {
  virtuoso: Ref<VirtuosoHandle>;
  documents: (Document | SearchResultDocument)[];
  selectionIndex: number;
  isMenuOpen: MutableRefObject<boolean>;
  onTagClick: (tag: string) => void;
  onRowClick: (index: number) => void;
  onEndReached: (index: number) => void;
};

export const DocumentList = React.memo(
  forwardRef<HTMLDivElement, Props>(
    (
      {
        virtuoso,
        documents,
        selectionIndex,
        isMenuOpen,
        onTagClick,
        onRowClick,
        onEndReached,
      },
      ref,
    ) => {
      return (
        <Column
          ref={ref}
          css={{
            overflowY: 'scroll',
          }}
        >
          <Virtuoso
            ref={virtuoso}
            style={{ height: 'calc(100vh - 40px)' }}
            totalCount={documents.length}
            endReached={onEndReached}
            // increaseViewportBy={200}
            itemContent={(i) => {
              const document = documents[i];
              return (
                <DocumentRow
                  key={document.path}
                  document={document}
                  isSelected={selectionIndex === i}
                  isMenuOpen={isMenuOpen}
                  onTagClick={onTagClick}
                  onClick={() => onRowClick(i)}
                />
              );
            }}
          ></Virtuoso>
        </Column>
      );
    },
  ),
);

DocumentList.displayName = 'DocumentList';
