import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { WatchState } from '../models/Watch';
import { getWatchState } from '../api/core';
import { useQueryClient } from 'react-query';

export const useWatchState = (): WatchState => {
  const [state, setState] = useState<WatchState>({
    watches: [],
    jobReports: [],
  });
  const queryClient = useQueryClient();

  useEffect(() => {
    (async () => {
      const state = await getWatchState();
      if (state.jobReports.some((report) => report.status === 'finished')) {
        queryClient.invalidateQueries(['documents']);
      }
      setState(state);
    })();
  }, []);

  useEffect(() => {
    const f = async () => {
      const unlisten = await listen<WatchState>('watches', (event) => {
        setState(event.payload);
      });

      return () => {
        unlisten();
      };
    };
    f();
  }, []);

  return state;
};
