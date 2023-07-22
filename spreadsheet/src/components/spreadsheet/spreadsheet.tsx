/* eslint-disable react-hooks/exhaustive-deps */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import { evaluate } from 'mathjs';
import { useCallback, useEffect, useRef, useState } from 'react';
import { Table, TableBody, TableHead } from './spreadsheet.style';
import { TypeOfMessage } from '../alerts/types';
import { v4 as uuidv4 } from 'uuid';
import { useNotificationContext } from '../../context/notificationContext';
import { debounce } from 'lodash';
interface CellValues {
  [cell: string]: {
    value: string;
    formula: string | null;
  };
}

enum Status {
  DONE = 'DONE',
  IN_PROGRESS = 'IN_PROGRESS',
}

interface ISpreadsheetProps {
  rows: number;
  columns: number;
}

interface POSTResponse {
  id?: string;
  status?: Status;
  done_at?: string;
}

interface GETResponse {
  id?: string;
  status: Status;
  done_at?: string;
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

      try {
        return evaluate(evaluatedFormula) as number;
      } catch (error) {
        console.error('Error evaluating formula:', error);
        return '#ERROR';
      }
    },
    [cellValues],
  );

  const handleCellValueChange = (cell: string, value: string) => {
    updateCellValue(cell, value);

    debouncedPostDataToServer(postDataToServer);
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
          const result = evaluateFormula(formula.slice(1));
          updateCellValue(
            cell,
            typeof result === 'number' ? parseFloat(result.toFixed(2)).toString() : result,
          );
        }
      });

      prevCellValues.current = cellValues;
    }
  }, [cellValues, focusedCell, evaluateFormula]);

  const convertToCSV = useCallback((data: CellValues, rows: number, columns: number): string => {
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
  }, []);

  const checkStatus = useCallback((id: string) => {
    fetch(`http://localhost:8082/get-status?id=${id}`)
      .then((response) => response.json())
      .then((data: GETResponse) => {
        if (data.status === Status.DONE) {
          setSelectedNotification({
            type_of_message: TypeOfMessage.SUCCESS,
            title: 'Procesare finalizată cu succes!',
            uid: uuidv4(),
          });
        } else if (data.status === Status.IN_PROGRESS) {
          setSelectedNotification({
            type_of_message: TypeOfMessage.INFO,
            title: 'Procesare în curs. Verificare starea din nou mai târziu...',
            uid: uuidv4(),
          });
          setTimeout(() => checkStatus(id), 5000);
        }
      })
      .catch(() => {
        console.error('PULA');
      });
  }, []);

  const postDataToServer = useCallback(() => {
    const csvData = convertToCSV(cellValues, rows, columns);
    fetch('http://localhost:8082/save', {
      method: 'POST',
      headers: {
        'Content-Type': 'text/csv',
      },
      body: csvData,
    })
      .then((response) => {
        if (response.status === 200) {
          return response.json();
        } else {
          setSelectedNotification({
            type_of_message: TypeOfMessage.ERROR,
            title: 'Eroare la salvarea datelor pe server!',
            uid: uuidv4(),
          });
        }
      })
      .then((data: POSTResponse) => {
        if (data.status === Status.DONE) {
          setSelectedNotification({
            type_of_message: TypeOfMessage.SUCCESS,
            title: 'Procesare finalizată cu succes!',
            uid: uuidv4(),
          });
        } else if (data.status === Status.IN_PROGRESS) {
          if (data.id) {
            checkStatus(data.id);
          }
        }
      })
      .catch(() => {
        setSelectedNotification({
          type_of_message: TypeOfMessage.ERROR,
          title: 'Eroare la salvarea datelor',
          uid: uuidv4(),
        });
      });
  }, []);

  const debouncedPostDataToServer = useRef(
    debounce((postDataToServer) => {
      console.log('Salvare date...');
      postDataToServer();
    }, 2000), // Specificăm intervalul de 2 secunde pentru debounce
  ).current;

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
          );
        })}
      </TableBody>
    </Table>
  );
};

export default Spreadsheet;
