import styled from 'styled-components';

export const Table = styled.table`
  width: 100%;
`;

export const TableHead = styled.thead`
  &:after {
    content: '-';
    display: block;
    line-height: 0.5rem;
    color: transparent;
  }

  tr {
    height: 2rem;

    display: flex;
    align-items: center;

    background-color: var(--color-gray);

    th {
      input {
        width: 100%;
      }

      &:nth-child(1),
      &:nth-child(2) {
        flex-basis: 40%;
      }

      &:nth-child(3) {
        flex-basis: 60%;
      }
    }
  }
`;

export const TableBody = styled.tbody`
  tr {
    height: 2rem;

    display: flex;
    align-items: center;

    margin-bottom: 0.25rem;

    &.error {
      border: 2px solid var(--color-red);
      border-radius: 4px;

      input {
        background-color: var(--color-red-transparent);
      }
    }

    td {
      height: 100%;

      &:nth-child(1),
      &:nth-child(2) {
        flex-basis: 40%;
        position: relative;

        &:after {
          position: absolute;
          content: '';
          width: 0.38px;
          height: 87.5%;
          background-color: var(--color-black-transparent);
          top: 0.125rem;
          right: 0;
        }
      }

      &:nth-child(3) {
        flex-basis: 60%;
      }
    }
  }
`;

export const TableInput = styled.input<{
  icon: string;
}>`
  height: 100%;
  width: 100%;

  text-align: center;
  border: none;

  background-color: var(--color-white);
  background-image: url(${(props) => props.icon});
  background-repeat: no-repeat;
  background-position: 98% 85%;
`;
