import React, { MutableRefObject, Ref } from 'react';
import { DocumentList } from './DocumentList';
import { SearchResults, Sort } from '../../models/Document';
import { Column } from '../../components/Column';
import { SearchResultsStats } from './SearchResultsStats';
import { SearchResultsSkeleton } from './SearchResultsSkeleton';
import { VirtuosoHandle } from 'react-virtuoso';

export const SearchResultsPane: React.FC<{
  isLoading: boolean;
  showSort: boolean;
  sort: Sort;
  searchResults?: SearchResults;
  virtuoso: Ref<VirtuosoHandle>;
  selectionIndex: number;
  isMenuOpen: MutableRefObject<boolean>;
  onTagClick: (tag: string) => void;
  onRowClick: (index: number) => void;
  onSortChange: (sort: Sort) => void;
  onEndReached: (index: number) => void;
}> = ({
  isLoading,
  showSort,
  sort,
  searchResults,
  virtuoso,
  selectionIndex,
  isMenuOpen,
  onRowClick,
  onTagClick,
  onSortChange,
  onEndReached,
}) => {
  if (isLoading) {
    return <SearchResultsSkeleton />;
  }
  if (searchResults == null) {
    return null;
  }

  return (
    <Column
      css={{
        overflowY: 'hidden',
      }}
    >
      <SearchResultsStats
        count={searchResults.count}
        showSort={showSort}
        sort={sort}
        onSortChange={onSortChange}
      />
      {searchResults.documents.length > 0 && (
        <DocumentList
          virtuoso={virtuoso}
          documents={searchResults.documents}
          selectionIndex={selectionIndex}
          isMenuOpen={isMenuOpen}
          onTagClick={onTagClick}
          onRowClick={onRowClick}
          onEndReached={onEndReached}
        />
      )}
    </Column>
  );
};
