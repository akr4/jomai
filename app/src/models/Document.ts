export type Document = {
  path: string;
  title?: string;
  tags: string[];
  createdAt: string;
  modifiedAt: string;
};

export const getTitle = (document: Document): string => {
  const getLastPartOfPath = (path: string): string | undefined => {
    return path.split('/').pop();
  };
  const removeExtension = (path?: string): string | undefined => {
    return path?.split('.').slice(0, -1).join('.');
  };

  if (document.title != null) {
    return document.title;
  }
  const title = removeExtension(getLastPartOfPath(document.path));
  return title != null ? title : 'Untitled';
};

export type SearchResultDocument = Document & {
  highlight: string | undefined;
};

export type SearchResults = {
  count: number;
  documents: SearchResultDocument[];
};

export const isSearchResultDocument = (
  document: Document,
): document is SearchResultDocument => {
  return Object.prototype.hasOwnProperty.call(document, 'highlight');
};

export const ALL_SORTS = ['relevance', 'date'];
export type Sort = typeof ALL_SORTS[number];
