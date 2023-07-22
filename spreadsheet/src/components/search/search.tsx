import { SearchHeader, SearchInput, SearchWrapper } from './search.style';
import iconSearch from '../../assets/iconSearch.svg';
interface ISearchProps {
  onChange: React.ChangeEventHandler;
}

const Search = ({ onChange }: ISearchProps) => {
  return (
    <SearchWrapper>
      <SearchHeader>Your Personal Staking Calculator</SearchHeader>
      <SearchInput
        name='search'
        type='text'
        placeholder='Type a search query to filter'
        onChange={onChange}
        icon={iconSearch}
      />
    </SearchWrapper>
  );
};

export default Search;
