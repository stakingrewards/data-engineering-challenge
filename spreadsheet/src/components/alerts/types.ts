export enum TypeOfMessage {
  ERROR = 'ERROR',
  SUCCESS = 'SUCCESS',
  INFO = 'INFO',
}

export interface INotificationMessage {
  title: string;
  type_of_message: TypeOfMessage;
}
