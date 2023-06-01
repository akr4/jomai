import React, { useCallback } from 'react';
import { QueryClient, QueryClientProvider } from 'react-query';
import { usePathRecommendations } from './helpers/usePathRecommendations';
import { PathRecommendation } from './models/Watch';
import { MainScreen } from './main/MainScreen';
import { PathRecommendationScreen } from './pathRecommendation/PathRecommendationScreen';
import { globalStyles } from './components/stitches.config';
import { ThemeProvider } from './components/ThemeProvider';
import * as Toast from './components/Toast';
import * as Tooltip from './components/Tooltip';

const queryClient = new QueryClient();

function App() {
  const [pathRecommendations, setRecommendationDone] = usePathRecommendations();
  globalStyles();

  const handleOnRecommendationFlowDone = useCallback(
    (pathRecommendations: PathRecommendation[]) => {
      setRecommendationDone(pathRecommendations);
    },
    [],
  );

  if (pathRecommendations.length > 0) {
    return (
      <ThemeProvider>
        <QueryClientProvider client={queryClient}>
          <PathRecommendationScreen
            pathRecommendations={pathRecommendations}
            onDone={handleOnRecommendationFlowDone}
          />
        </QueryClientProvider>
      </ThemeProvider>
    );
  }

  return (
    <ThemeProvider>
      <Tooltip.Provider>
        <Toast.Provider>
          <QueryClientProvider client={queryClient}>
            <MainScreen />
            <Toast.Viewport />
          </QueryClientProvider>
        </Toast.Provider>
      </Tooltip.Provider>
    </ThemeProvider>
  );
}

export default App;
