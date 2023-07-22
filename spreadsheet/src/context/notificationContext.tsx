/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, useMemo, useState } from 'react';
import { INotificationMessage } from '../components/alerts/types';

interface INotificationWrapperContext {
  children: React.JSX.Element;
}

interface INotificationContext {
  notificationSelected: INotificationMessage | undefined;
  setSelectedNotification: (notificationSelected: INotificationMessage) => void;
}

export const NotificationContext = createContext<INotificationContext>({
  notificationSelected: undefined,
  setSelectedNotification: () => null,
});

export const NotificationWrapper = (props: INotificationWrapperContext) => {
  const [notificationSelected, setSelectedNotification] = useState<INotificationMessage>();
  const { children } = props;

  return (
    <NotificationContext.Provider
      value={useMemo<INotificationContext>(() => {
        return {
          notificationSelected,
          setSelectedNotification,
        } as INotificationContext;
      }, [notificationSelected, setSelectedNotification])}
    >
      {children}
    </NotificationContext.Provider>
  );
};

export const useNotificationContext = () => useContext(NotificationContext);
