import React, { ChangeEvent } from 'react';
import { TableHeader } from './grid.style';

interface GridProps {
  rows: number;
  columns: number;
  cellValues: string[][];
  onCellValueChange: (row: number, column: number, value: string) => void;
}

const Grid: React.FC<GridProps> = ({ rows, columns, cellValues, onCellValueChange }) => {
  const handleCellValueChange = (
    event: ChangeEvent<HTMLInputElement>,
    row: number,
    column: number,
  ) => {
    if (onCellValueChange) {
      const newValue = event.target.value;
      onCellValueChange(row, column, newValue);
    }
  };

  console.log(cellValues);

  return (
    <table>
      <TableHeader>
        <th>A</th>
        <th>B</th>
        <th>C</th>
      </TableHeader>
      <tbody>
        {Array.from({ length: rows }).map((_, row) => (
          <tr key={row}>
            {Array.from({ length: columns }).map((_, column) => (
              <td key={column}>
                <input
                  type='text'
                  value={cellValues[row][column]}
                  onChange={(event) => handleCellValueChange(event, row, column)}
                />
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default Grid;
