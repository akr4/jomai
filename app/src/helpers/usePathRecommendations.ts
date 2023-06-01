import { useCallback, useEffect, useState } from 'react';
import { addWatch, getAllWatches, getPathRecommendations } from '../api/core';
import { PathRecommendation } from '../models/Watch';

export const usePathRecommendations = (): [
  PathRecommendation[],
  (pathRecommendations: PathRecommendation[]) => void,
] => {
  const [done, setDone] = useState(false);
  const [pathRecommendations, setPathRecommendations] = useState<
    PathRecommendation[]
  >([]);

  const onDone = useCallback((pathRecommendations: PathRecommendation[]) => {
    console.log(JSON.stringify({ pathRecommendations }, null, 2));
    setDone(true);
    setPathRecommendations([]);
    pathRecommendations.forEach((r) => {
      addWatch(r.path);
    });
  }, []);

  useEffect(() => {
    const f = async () => {
      const watches = await getAllWatches();
      if (watches.length === 0) {
        const x = await getPathRecommendations();
        setPathRecommendations(x);
      }
    };
    f();
  }, [done]);

  return [pathRecommendations, onDone];
};
