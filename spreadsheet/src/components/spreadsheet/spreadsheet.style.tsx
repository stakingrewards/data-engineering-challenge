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
    display: flex;
    align-items: center;

    height: 2rem;
    background-color: #efefef;

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
    display: flex;
    align-items: center;

    height: 2rem;
    margin-bottom: 0.25rem;

    td {
      height: 100%;

      input {
        border: none;
        background-color: #fafafa;
        height: 100%;
        width: 100%;
        text-align: center;
      }

      &:nth-child(1),
      &:nth-child(2) {
        flex-basis: 40%;
        position: relative;

        &:after {
          position: absolute;
          content: '';
          width: 0.38px;
          height: 87.5%;
          background-color: rgba(0, 0, 0, 0.3);
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
