import React, { useCallback } from 'react';
import { BasicLayout } from '../layout/BasicLayout';
import { Column } from '../components/Column';
import { PathRecommendation } from '../models/Watch';
import { PathRecommendationList } from './PathRecommendationList';
import { Footer } from './Footer';
import { Header } from './Header';
import { Spacer } from '../components/Spacer';

export const PathRecommendationScreen: React.FC<{
  pathRecommendations: PathRecommendation[];
  onDone: (pathRecommendations: PathRecommendation[]) => void;
}> = ({ pathRecommendations, onDone }) => {
  const [selected, setSelected] = React.useState<PathRecommendation[]>([]);

  const handleSelect = useCallback(
    (pathRecommendation: PathRecommendation, isSelect: boolean) => {
      if (isSelect) {
        setSelected((selected) => [...selected, pathRecommendation]);
      } else {
        setSelected((selected) =>
          selected.filter((pr) => pr.type !== pathRecommendation.type),
        );
      }
    },
    [selected],
  );

  const handleNextClick = useCallback(() => {
    onDone(selected);
  }, [selected]);

  return (
    <BasicLayout>
      <Column
        css={{
          padding: '$large',
        }}
      >
        <Header />
        <Spacer height="large" />
        <PathRecommendationList
          pathRecommendations={pathRecommendations}
          selectedPathRecommendations={selected}
          onSelect={handleSelect}
        />
        <Spacer height="large" />
        <Footer onNextClick={handleNextClick} />
      </Column>
    </BasicLayout>
  );
};
