/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import { evaluate } from 'mathjs';
import { useCallback, useEffect, useRef, useState } from 'react';
import { Table, TableBody, TableHead } from './spreadsheet.style';

interface CellValues {
  [cell: string]: {
    value: string;
    formula: string | null;
  };
}

interface ISpreadsheetProps {
  rows: number;
  columns: number;
}

const Spreadsheet = ({ rows = 50, columns = 3 }: ISpreadsheetProps) => {
  const [cellValues, setCellValues] = useState<CellValues>(() => {
    const initialCellValues: CellValues = {};

    for (let row = 1; row <= rows; row++) {
      for (let col = 1; col <= columns; col++) {
        const cell = String.fromCharCode(64 + col) + row;
        initialCellValues[cell] = {
          value: '',
          formula: null,
        };
      }
    }

    return initialCellValues;
  });

  const [focusedCell, setFocusedCell] = useState<boolean>(false);
  const prevCellValues = useRef<CellValues>(cellValues);

  const updateCellValue = (cell: string, value: string) => {
    setCellValues((prevCellValues) => ({
      ...prevCellValues,
      [cell]: {
        ...prevCellValues[cell],
        value,
      },
    }));
  };

  const updateCellFormula = (cell: string, formula: string) => {
    setCellValues((prevCellValues) => ({
      ...prevCellValues,
      [cell]: {
        ...prevCellValues[cell],
        formula,
      },
    }));
  };

  const evaluateFormula = useCallback(
    (formula: string): string | number => {
      const match = formula.match(/([A-Z])(\d+)/g);
      let evaluatedFormula = formula;
      match?.forEach((match) => {
        const [cell, row] = reduceZeros(match);
        const cellValue = cellValues[`${cell}${row}`]?.value || '0';

        evaluatedFormula = evaluatedFormula.replace(match, cellValue.replace(/[^\d.%]/g, ''));
      });
      console.log(evaluatedFormula);

      try {
        return evaluate(evaluatedFormula);
      } catch (error) {
        console.error('Error evaluating formula:', error);
        return '#ERROR';
      }
    },
    [cellValues],
  );

  const handleCellValueChange = (cell: string, value: string) => {
    updateCellValue(cell, value);
  };

  const reduceZeros = (input: string) => {
    const letterPart = input.charAt(0);
    const numericPart = parseInt(input.slice(1)).toString();

    return [letterPart, numericPart];
  };

  const handleBlur = (cell: string) => {
    const value = cellValues[cell].value;

    if (value.startsWith('=')) {
      const result = evaluateFormula(value.slice(1));
      updateCellFormula(cell, value);

      updateCellValue(
        cell,
        typeof result === 'number' ? parseFloat(result.toFixed(2)).toString() : result,
      );
    }
    setFocusedCell(false);
  };

  const handleFocus = (cell: string) => {
    setFocusedCell(true);
    const formula = cellValues[cell].formula;
    if (formula) {
      updateCellValue(cell, formula);
    }
  };

  useEffect(() => {
    if (focusedCell) return;

    const hasChanges = Object.entries(cellValues).some(([cell, { formula, value }]) => {
      const prevCellValue = prevCellValues.current[cell]?.value;
      const prevCellFormula = prevCellValues.current[cell]?.formula;

      return value !== prevCellValue || formula !== prevCellFormula;
    });

    if (hasChanges) {
      Object.entries(cellValues).forEach(([cell, { formula }]) => {
        if (formula) {
          const result = evaluateFormula(formula.slice(1));
          console.log(result);

          updateCellValue(
            cell,
            typeof result === 'number' ? parseFloat(result.toFixed(2)).toString() : result,
          );
        }
      });

      prevCellValues.current = cellValues;
    }
  }, [cellValues, focusedCell, evaluateFormula]);

  return (
    <Table>
      <TableHead>
        <tr>
          {[...Array(columns)].map((_, colIndex) => {
            return (
              <th key={colIndex} scope='col'>
                {String.fromCharCode(65 + colIndex)}
              </th>
            );
          })}
        </tr>
      </TableHead>

      <TableBody>
        {[...Array(rows)].map((_, rowIndex) => (
          <tr key={rowIndex}>
            {[...Array(columns)].map((_, colIndex) => {
              const cell = String.fromCharCode(65 + colIndex) + (rowIndex + 1).toString();
              const cellValue = cellValues[cell].value || '';

              return (
                <td key={cell}>
                  <input
                    type='text'
                    value={cellValue}
                    onChange={(e) => handleCellValueChange(cell, e.target.value)}
                    onBlur={() => handleBlur(cell)}
                    onFocus={() => handleFocus(cell)}
                  />
                </td>
              );
            })}
          </tr>
        ))}
      </TableBody>
    </Table>
  );
};

export default Spreadsheet;
