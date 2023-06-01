import { appWindow, Theme } from '@tauri-apps/api/window';
import React, { ReactNode, useEffect, useState } from 'react';
import { UnlistenFn } from '@tauri-apps/api/event';
import { darkTheme, lightTheme } from './stitches.config';

export const useTheme = () => {
  const [theme, setTheme] = useState<typeof lightTheme | typeof darkTheme>(
    lightTheme,
  );

  const updateThemeByName = (theme: Theme | null) => {
    if (theme === 'dark') {
      setTheme(darkTheme);
    } else {
      setTheme(lightTheme);
    }
  };

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    (async () => {
      updateThemeByName(await appWindow.theme());

      unlisten = await appWindow.onThemeChanged(({ payload: theme }) => {
        console.log(`theme changed to ${theme}`);
        updateThemeByName(theme);
      });
    })();

    return () => {
      if (unlisten != null) {
        unlisten();
      }
    };
  }, []);

  return theme;
};

export const ThemeProvider = ({ children }: { children: ReactNode }) => {
  const theme = useTheme();
  document.body.className = theme.className;
  return <>{children}</>;
};
