import { useState } from 'react';
import Search from './components/search/search';
import Spreadsheet from './components/spreadsheet/spreadsheet';
import { Alerts } from './components/alerts/alerts';
import { NotificationWrapper } from './context/notificationContext';

const App = () => {
  const [inputValue, setInputValue] = useState('');

  console.log(inputValue);

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
