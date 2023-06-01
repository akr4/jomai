import { SearchResults, Sort } from '../models/Document';
import { invoke } from '@tauri-apps/api/tauri';
import { PathRecommendation, Watch, WatchState } from '../models/Watch';

export const getAllWatches = (): Promise<Watch[]> => {
  return invoke('get_all_watches');
};

export const getWatchState = (): Promise<WatchState> => {
  return invoke('get_watch_state');
};

export type AddWatchError =
  | { type: 'parent-child-relationship' }
  | { type: 'watch-already-exists' }
  | { type: 'other' };
export const addWatch = (path: string): Promise<Watch> => {
  return invoke('add_watch', { path });
};

export const deleteWatch = (path: string): Promise<void> => {
  return invoke('delete_watch', { path });
};

export const getAllDocuments = (
  offset: number,
  limit: number,
): Promise<SearchResults> => {
  return invoke('get_all_documents', { offset, limit });
};

export const searchDocuments = (
  query: string,
  tags: string[],
  sort: Sort,
  offset: number,
  limit: number,
): Promise<SearchResults> => {
  return invoke('search_documents', { query, tags, sort, offset, limit });
};

export const getContainingFolder = (path: string): Promise<string> => {
  return invoke('get_containing_folder', { path });
};

export const shutdown = () => {
  return invoke('shutdown');
};

export const getPathRecommendations = (): Promise<PathRecommendation[]> => {
  return invoke('get_path_recommendations');
};
