import { AlertBlock, AlertWrapper } from './alerts.style';
import { useNotificationContext } from '../../context/notificationContext';

export const Alerts = () => {
  const { notificationSelected } = useNotificationContext();

  if (!notificationSelected) {
    return null;
  }

  return (
    <AlertBlock>
      <AlertWrapper
        key={notificationSelected.title}
        type_of_message={notificationSelected.type_of_message}
      >
        <p>{notificationSelected.title}</p>
      </AlertWrapper>
    </AlertBlock>
  );
};
