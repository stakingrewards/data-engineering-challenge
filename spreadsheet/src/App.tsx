import { useState } from 'react';
import Search from './components/search/search';
import Spreadsheet from './components/spreadsheet/spreadsheet';
import { Alerts } from './components/alerts/alerts';
import { NotificationWrapper } from './context/notificationContext';

const App = () => {
  // It's for the search component, but it's not used
  const [, setInputValue] = useState('');

  return (
    <NotificationWrapper>
      <main>
        <Search
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            setInputValue(e.target.value);
          }}
        />
        <Spreadsheet rows={50} columns={3} />

        <Alerts />
      </main>
    </NotificationWrapper>
  );
};

export default App;
