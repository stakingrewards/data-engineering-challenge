export enum TypeOfMessage {
  ERROR = 'ERROR',
  SUCCESS = 'SUCCESS',
  INFO = 'INFO',
}

export interface INotificationMessage {
  uid: string;
  title: string;
  type_of_message: TypeOfMessage;
}
