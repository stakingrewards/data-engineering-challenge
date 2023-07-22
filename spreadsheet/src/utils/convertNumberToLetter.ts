import { evaluate } from 'mathjs';
import { CellValues } from '../components/spreadsheet/spreadsheet';

export const reduceZeros = (input: string) => {
  const letterPart = input.charAt(0);
  const numericPart = parseInt(input.slice(1)).toString();

  return [letterPart, numericPart];
};

export const convertToCSV = (data: CellValues, rows: number, columns: number): string => {
  const csvRows: string[] = [];

  for (let row = 1; row <= rows; row++) {
    const rowData: string[] = [];
    for (let col = 1; col <= columns; col++) {
      const cell = String.fromCharCode(64 + col) + row;
      const cellValue = data[cell]?.value || '';
      rowData.push(cellValue);
    }
    csvRows.push(rowData.join(','));
  }
  return csvRows.join('\n');
};

export const evaluateFormula = (formula: string, cellValues: CellValues): string | number => {
  const match = formula.match(/([A-Z])(\d+)/g);
  let evaluatedFormula = formula;
  match?.forEach((match) => {
    const [cell, row] = reduceZeros(match);
    const cellValue = cellValues[`${cell}${row}`]?.value || '0';

    evaluatedFormula = evaluatedFormula.replace(match, cellValue.replace(/[^\d.%]/g, ''));
  });

  try {
    return evaluate(evaluatedFormula) as number;
  } catch (error) {
    console.error('Error evaluating formula:', error);
    return '#ERROR';
  }
};
