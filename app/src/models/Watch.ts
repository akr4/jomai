export type Watch = {
  id: number;
  path: string;
  status: 'active' | 'adding' | 'deleting';
  createdAt: string;
};

export type WatchFull = Watch & {
  documentCount: number;
};

export type WatchJobReport = {
  watch: Watch;
  progress: {
    done: number;
    total: number;
  };
  jobType: 'scan_watch_path' | 'delete_watch' | 'sync_watch';
  status: 'running' | 'finished';
};

export type WatchState = {
  watches: WatchFull[];
  jobReports: WatchJobReport[];
};

export type PathRecommendation = {
  path: string;
  type: 'documents' | 'obsidian';
};
