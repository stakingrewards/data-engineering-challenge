import { SearchHeader, SearchInput, SearchWrapper } from './search.style';

interface ISearchProps {
  onChange: React.ChangeEventHandler;
}

const Search = ({ onChange }: ISearchProps) => {
  return (
    <SearchWrapper>
      <SearchHeader>Your Personal Staking Calculator</SearchHeader>
      <SearchInput type='text' placeholder='Type a search query to filter' onChange={onChange} />
    </SearchWrapper>
  );
};

export default Search;
