import styled from 'styled-components';

export const SearchWrapper = styled.div`
  display: flex;
  flex-direction: column;

  margin-top: 2rem;
`;

export const SearchHeader = styled.h1`
  font-weight: 700;
  font-size: 1.25rem;
  line-height: 1.5rem;
`;

export const SearchInput = styled.input<{
  icon: string;
}>`
  width: 100%;
  height: 2rem;
  margin: 0.25rem 0 0.875rem;

  border: none;
  border-radius: 5px;
  font-size: 0.75rem;

  background-color: var(--color-searchBox-gray);

  background-image: url(${(props) => props.icon});
  background-repeat: no-repeat;
  background-position: 0.5rem center;
  background-size: 1rem;
  padding: 0.5rem 0 0.5rem 2rem;
`;
