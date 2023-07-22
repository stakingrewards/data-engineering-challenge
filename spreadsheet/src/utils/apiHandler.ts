/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { INotificationMessage, TypeOfMessage } from '../components/alerts/types';
import { CellValues } from '../components/spreadsheet/spreadsheet';
import { convertToCSV } from './convertNumberToLetter';

enum Status {
  DONE = 'DONE',
  IN_PROGRESS = 'IN_PROGRESS',
}

interface POSTResponse {
  id?: string;
  status?: Status;
  done_at?: string;
}

interface GET {
  id?: string;
  status: Status;
  done_at?: string;
}

export const checkStatus = (
  id: string,
  setSelectedNotification: (notificationSelected: INotificationMessage) => void,
) => {
  fetch(`http://localhost:8082/get-status?id=${id}`)
    .then((response) => response.json())
    .then((data: GET) => {
      if (data.status === Status.DONE) {
        setSelectedNotification({
          type_of_message: TypeOfMessage.SUCCESS,
          title: 'Processing completed successfully!',
        });
      } else if (data.status === Status.IN_PROGRESS) {
        setSelectedNotification({
          type_of_message: TypeOfMessage.INFO,
          title: 'Processing in progress. Check status again later...',
        });
        setTimeout(() => checkStatus(id, setSelectedNotification), 5000);
      }
    })
    .catch((error) => {
      console.error('We find an error:', error);
    });
};

export const postDataToServer = (
  cellValues: CellValues,
  rows: number,
  columns: number,
  setSelectedNotification: (notificationSelected: INotificationMessage) => void,
) => {
  const csvData = convertToCSV(cellValues, rows, columns);
  fetch(`http://localhost:8082/save`, {
    method: 'POST',
    headers: {
      'Content-Type': 'text/csv',
    },
    body: csvData,
  })
    .then((response) => {
      if (response.status === 200) {
        return response.json();
      }
    })
    .then((data: POSTResponse) => {
      if (data.status === Status.DONE) {
        setSelectedNotification({
          type_of_message: TypeOfMessage.SUCCESS,
          title: 'Processing completed successfully!',
        });
      } else if (data.status === Status.IN_PROGRESS) {
        if (data.id) {
          checkStatus(data.id, setSelectedNotification);
        }
      }
    })
    .catch(() => {
      postDataToServer(cellValues, rows, columns, setSelectedNotification);

      setSelectedNotification({
        type_of_message: TypeOfMessage.ERROR,
        title: 'Error saving data. Try again...',
      });
    });
};
