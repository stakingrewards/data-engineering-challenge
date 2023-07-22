/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import { useEffect, useRef, useState } from 'react';
import { Table, TableBody, TableHead, TableInput } from './spreadsheet.style';
import { useNotificationContext } from '../../context/notificationContext';
import { debounce } from 'lodash';
import pencil from '../../assets/pencil.svg';
import { evaluateFormula } from '../../utils/convertNumberToLetter';
import { postDataToServer } from '../../utils/apiHandler';
import { INotificationMessage } from '../alerts/types';
export interface CellValues {
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
    const getDataFromLocalStorage = localStorage.getItem('spreadsheetData');

    if (getDataFromLocalStorage) {
      return JSON.parse(getDataFromLocalStorage) as CellValues;
    }

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
  const { setSelectedNotification } = useNotificationContext();
  const debouncedPostDataToServer = useRef(
    debounce(
      (
        cellValues: CellValues,
        rows: number,
        columns: number,
        setSelectedNotification: (notificationSelected: INotificationMessage) => void,
      ) => {
        postDataToServer(cellValues, rows, columns, setSelectedNotification);
      },
      2000,
    ),
  ).current;

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

  const handleCellValueChange = (cell: string, value: string) => {
    updateCellValue(cell, value);

    debouncedPostDataToServer(cellValues, rows, columns, setSelectedNotification);
  };

  const handleBlur = (cell: string) => {
    const value = cellValues[cell].value;

    if (value.startsWith('=')) {
      const result = evaluateFormula(value.slice(1), cellValues);
      updateCellFormula(cell, value);

      updateCellValue(
        cell,
        typeof result === 'number' ? parseFloat(result.toFixed(2)).toString() : result,
      );
    }
    setFocusedCell(false);
  };

  const handleFocus = (cell: string) => {
    const formula = cellValues[cell].formula;
    if (formula) {
      updateCellValue(cell, formula);
    }
    setFocusedCell(true);
  };

  useEffect(() => {
    localStorage.setItem('spreadsheetData', JSON.stringify(cellValues));
  }, [cellValues]);

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
          const result = evaluateFormula(formula.slice(1), cellValues);
          updateCellValue(
            cell,
            typeof result === 'number' ? parseFloat(result.toFixed(2)).toString() : result,
          );
        }
      });

      prevCellValues.current = cellValues;
    }
  }, [cellValues, focusedCell]);

  useEffect(() => {
    return () => {
      debouncedPostDataToServer.cancel();
    };
  }, [debouncedPostDataToServer]);

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
        {[...Array(rows)].map((_, rowIndex) => {
          const hasError = [...Array(columns)].some((_, colIndex) => {
            const cell = String.fromCharCode(65 + colIndex) + (rowIndex + 1).toString();
            const cellValue = cellValues[cell].value || '';
            return cellValue === '#ERROR';
          });

          return (
            <tr key={rowIndex} className={hasError ? 'error' : ''}>
              {[...Array(columns)].map((_, colIndex) => {
                const cell = String.fromCharCode(65 + colIndex) + (rowIndex + 1).toString();
                const cellValue = cellValues[cell].value || '';

                return (
                  <td key={cell}>
                    <TableInput
                      name='cell'
                      icon={pencil}
                      type='text'
                      value={cellValue}
                      onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                        handleCellValueChange(cell, e.target.value)
                      }
                      onBlur={() => handleBlur(cell)}
                      onFocus={() => handleFocus(cell)}
                    />
                  </td>
                );
              })}
            </tr>
          );
        })}
      </TableBody>
    </Table>
  );
};

export default Spreadsheet;
