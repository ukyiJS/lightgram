import React from 'react';
import ReactDOM from 'react-dom/client';

import IndexPage from '@/pages/index/ui/index-page';
import '@/styles/globals.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <IndexPage />
  </React.StrictMode>,
);
