import { useMutation, useQueryClient } from 'react-query';
import { deleteWatch } from '../api/core';
import React from 'react';

export const useDeleteWatch = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation((path: string) => deleteWatch(path), {
    onSuccess: () => {
      queryClient.invalidateQueries(['watches']);
    },
  });
  const [error, setError] = React.useState<Error | undefined>(undefined);

  return (path: string) => {
    mutation.mutate(path);
  };
};
