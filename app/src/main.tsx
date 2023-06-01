import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './dayjs';
import './i18n/i18next';
import { setupContextMenu } from './contextMenu';

setupContextMenu();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
