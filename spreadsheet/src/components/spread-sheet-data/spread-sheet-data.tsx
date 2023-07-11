import React, { useState } from 'react';
import Grid from '../grid/grid';

interface CellData {
  value: string;
  formula: string;
}

const SpreadsheetData: React.FC = () => {
  const [cellData, setCellData] = useState<CellData[][]>(() => {
    const rows = 10;
    const columns = 3;
    const initialData: CellData[][] = Array.from({ length: rows }, () =>
      Array.from({ length: columns }, () => ({ value: '', formula: '' })),
    );
    return initialData;
  });

  const handleCellValueChange = (row: number, column: number, value: string) => {
    setCellData((prevData) => {
      const newData = [...prevData];
      newData[row][column].value = value;
      return newData;
    });
  };

  //   const handleCellFormulaChange = (row: number, column: number, formula: string) => {
  //     setCellData((prevData) => {
  //       const newData = [...prevData];
  //       newData[row][column].formula = formula;
  //       return newData;
  //     });
  //   };

  return (
    <Grid
      rows={cellData.length}
      columns={cellData[0].length}
      cellValues={cellData.map((row) => row.map((cell) => cell.value))}
      onCellValueChange={handleCellValueChange}
    />
  );
};

export default SpreadsheetData;
