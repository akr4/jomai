import { useMutation, useQueryClient } from 'react-query';
import { addWatch, AddWatchError } from '../api/core';
import { useState } from 'react';
import { Watch } from '../models/Watch';

export const useAddWatch = () => {
  const queryClient = useQueryClient();
  const [error, setError] = useState<AddWatchError>();

  const mutation = useMutation<Watch, AddWatchError, string>(
    (path: string) => addWatch(path),
    {
      onSuccess: (watch) => {
        queryClient.invalidateQueries(['watches']);
      },
      onError: (e) => {
        console.log(JSON.stringify({ error: e }, null, 2));
        setError(e);
      },
    },
  );

  const addWatches = (paths: string[]) => {
    paths.forEach((path) => {
      mutation.mutate(path);
    });
  };

  return { addWatch: addWatches, error };
};
