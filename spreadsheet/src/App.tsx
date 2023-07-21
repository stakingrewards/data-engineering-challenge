import { useState } from 'react';
import Search from './components/search/search';
import Spreadsheet from './components/spreadsheet/spreadsheet';

const App = () => {
  const [inputValue, setInputValue] = useState('');

  console.log(inputValue);

  return (
    <>
      <Search
        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
          setInputValue(e.target.value);
        }}
      />
      <Spreadsheet rows={50} columns={3} />
    </>
  );
};

export default App;
