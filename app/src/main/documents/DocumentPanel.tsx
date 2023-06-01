import React, {
  Dispatch,
  RefObject,
  SetStateAction,
  useCallback,
  useEffect,
  useRef,
  useState,
} from 'react';
import { Column } from '../../components/Column';
import { SearchForm } from './SearchForm';
import { Divider } from '../../components/Divider';
import { useInfiniteQuery } from 'react-query';
import { getAllDocuments, searchDocuments } from '../../api/core';
import { isNoModifiers, useKey, UseKeyCallback } from '../../helpers/useKey';
import { SearchResultsPane } from './SearchResultsPane';
import { SearchResults, Sort } from '../../models/Document';
import { VirtuosoHandle } from 'react-virtuoso';
import { copyToClipboard, openContainingFolder, openFile } from './actions';

export const DocumentPanel: React.FC = () => {
  const [query, setQuery] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [sort, setSort] = useState<Sort>('relevance');
  const { isLoading, isError, data, error, fetchNextPage } = useDocumentQuery(
    query,
    tags,
    sort,
  );

  const virtuoso = useRef<VirtuosoHandle>(null);
  const queryInputRef = useRef<HTMLInputElement>(null);
  const isMenuOpen = useRef(false);
  const documentCount = useRef(0);

  const [selectionIndex, setSelectionIndex, movement] = useSelection(
    documentCount,
    isMenuOpen,
  );

  const pages = data?.pages ?? [];
  const totalCount = pages.length > 0 ? pages[0].count : 0;
  const documents = pages.flatMap((page) => page.documents);

  documentCount.current = documents.length ?? 0;

  const handleQueryChange = useCallback((newQuery: string) => {
    setQuery(newQuery);
  }, []);

  const handleTagClick = useCallback(
    (tag: string) => {
      if (!tags.includes(tag)) {
        setTags([...tags, tag]);
      }
    },
    [tags],
  );

  const handleTagRemove = useCallback(
    (tag: string) => {
      setTags(tags.filter((t) => t !== tag));
    },
    [tags],
  );

  const handleRowClick = useCallback((index: number) => {
    setSelectionIndex(index);
  }, []);

  const handleOpenDocument = useCallback(() => {
    const document = documents[selectionIndex];
    if (document != null) {
      openFile(document.path);
      return true;
    } else {
      return false;
    }
  }, [selectionIndex, documents]);

  const handleOpenContainingFolder = useCallback(() => {
    const document = documents[selectionIndex];
    if (document != null) {
      openContainingFolder(document.path);
      return true;
    } else {
      return false;
    }
  }, [selectionIndex, documents]);

  const handleCopyPathToClipboard = useCallback(() => {
    const document = documents[selectionIndex];
    if (document != null) {
      copyToClipboard(document.path);
      return true;
    } else {
      return false;
    }
  }, [selectionIndex, documents]);

  const handleInputFocus = useCallback(() => {
    setSelectionIndex(-1);
  }, []);

  const handleSortChange = useCallback((newSort: Sort) => {
    setSort(newSort);
  }, []);

  const resetQuery = useCallback(() => {
    setQuery('');
    setTags([]);
    setSelectionIndex(-1);
    virtuoso.current?.scrollToIndex({ index: 0 });
    return true;
  }, []);

  const handleEndReached = useCallback(() => {
    fetchNextPage();
  }, []);

  useKeyboardShortcuts(
    selectionIndex !== -1,
    handleOpenDocument,
    handleOpenContainingFolder,
    handleCopyPathToClipboard,
    resetQuery,
  );

  useEffect(() => {
    setSelectionIndex(-1);
    virtuoso.current?.scrollToIndex({ index: 0 });
  }, [query, sort]);

  useEffect(() => {
    if (selectionIndex == -1) {
      queryInputRef.current?.focus();
    } else {
      virtuoso.current?.scrollIntoView({
        index: selectionIndex,
        align: movement === 'up' ? 'start' : 'end',
      });
      queryInputRef.current?.blur();
    }
  }, [selectionIndex]);

  // ルートの <Column> の高さが決まらないと SearchResultsPane の高さが決まらないのでマウスでスクロールできない
  return (
    <Column
      css={{
        height: 'calc(100vh - 40px)',
      }}
    >
      <SearchForm
        query={query}
        tags={tags}
        inputRef={queryInputRef}
        onQueryChange={handleQueryChange}
        onTagRemove={handleTagRemove}
        onInputFocus={handleInputFocus}
      />
      <Divider />
      <SearchResultsPane
        virtuoso={virtuoso}
        isLoading={isLoading}
        showSort={query.trim().length > 0}
        sort={sort}
        searchResults={
          isError
            ? { count: 0, documents: [] }
            : { count: totalCount, documents }
        }
        selectionIndex={selectionIndex}
        isMenuOpen={isMenuOpen}
        onTagClick={handleTagClick}
        onRowClick={handleRowClick}
        onSortChange={handleSortChange}
        onEndReached={handleEndReached}
      />
    </Column>
  );
};

type Movement = 'up' | 'down';

const useSelection = (
  // handleDown が古い size を掴んだままになる問題があり、原因がわからなかったが size を ref にすることで回避した
  size: RefObject<number>,
  isMenuOpen: RefObject<boolean>,
): [number, Dispatch<SetStateAction<number>>, Movement | undefined] => {
  const [index, setIndex] = useState(-1);
  const [movement, setMovement] = useState<Movement | undefined>(undefined);

  const handleUp = useCallback(() => {
    setIndex((x) => {
      if (-1 < x) {
        return x - 1;
      } else {
        return x;
      }
    });
    setMovement('up');
  }, []);

  const handleDown = useCallback(() => {
    setIndex((x) => {
      if (x < size.current! - 1) {
        return x + 1;
      } else {
        return x;
      }
    });
    setMovement('down');
  }, [size]);

  const callback = useCallback<UseKeyCallback>(
    (code, modifiers) => {
      if (isMenuOpen.current) {
        return false;
      }
      const noModifiers = isNoModifiers(modifiers);
      if (
        (code === 'ArrowUp' && noModifiers) ||
        // allow 'k' in query input
        (index !== -1 && code === 'KeyK' && noModifiers) ||
        (code === 'KeyP' && modifiers.ctrl)
      ) {
        handleUp();
        return true;
      } else if (
        (code === 'ArrowDown' && noModifiers) ||
        // allow 'j' in query input
        (index !== -1 && code === 'KeyJ' && noModifiers) ||
        (code === 'KeyN' && modifiers.ctrl)
      ) {
        handleDown();
        return true;
      } else {
        return false;
      }
    },
    [handleUp, handleDown, index],
  );

  useKey(callback);

  return [index, setIndex, movement];
};

const useKeyboardShortcuts = (
  enabled: boolean,
  openDocument: () => boolean,
  openContainingFolder: () => boolean,
  copyPathToClipboard: () => boolean,
  resetQuery: () => boolean,
) => {
  const callback = useCallback<UseKeyCallback>(
    (code, modifiers) => {
      if (!enabled) {
        return false;
      }
      const noModifiers = isNoModifiers(modifiers);
      if (code === 'Enter') {
        return openDocument();
      } else if (code === 'KeyF' && noModifiers) {
        return openContainingFolder();
      } else if (code === 'KeyC' && noModifiers) {
        return copyPathToClipboard();
      } else if (code === 'Escape') {
        return resetQuery();
      } else {
        return false;
      }
    },
    [enabled, openDocument, openContainingFolder],
  );
  useKey(callback);
};

type QueryFn = (
  query: string,
  tags: string[],
  sort: Sort,
  offset: number,
) => Promise<SearchResults>;
const PAGE_SIZE = 10;
const getQueryFn = (useGetAll: boolean): QueryFn => {
  if (useGetAll) {
    return (query: string, tags: string[], sort: Sort, offset: number) =>
      getAllDocuments(offset, PAGE_SIZE);
  } else {
    return (query: string, tags: string[], sort: Sort, offset: number) =>
      searchDocuments(query, tags, sort, offset, PAGE_SIZE);
  }
};

const useDocumentQuery = (query: string, tags: string[], sort: Sort) => {
  return useInfiniteQuery(
    ['documents', query, tags, sort],
    async ({ pageParam: offset = 0 }) => {
      const queryFn = getQueryFn(
        query.trim().length === 0 && tags.length === 0,
      );
      return await queryFn(query, tags, sort, offset);
    },
    {
      keepPreviousData: true,
      getNextPageParam: (lastPage, allPages) => {
        const length = allPages.reduce(
          (acc, page) => acc + page.documents.length,
          0,
        );
        if (length < lastPage.count) {
          return length;
        } else {
          return undefined;
        }
      },
    },
  );
};
