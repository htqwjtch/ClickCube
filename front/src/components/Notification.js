import React, { useEffect } from "react";
import "./Notification.css";

const notificationIcons = {
  successIcon: '/assets/icons/notifications/success.png',
  warningIcon: '/assets/icons/notifications/warning.png',
  errorIcon: '/assets/icons/notifications/error.png'
};

const Notification = ({ message, type, onClose }) => {
  useEffect(() => {
    const timer = setTimeout(onClose, 4000);
    return () => clearTimeout(timer);
  }, [onClose]);

  let notificationClass = "notification";
  let iconSrc = "";

  if (type === "success") {
    notificationClass += " success";
    iconSrc = notificationIcons.successIcon;
  } else if (type === "warning") {
    notificationClass += " warning";
    iconSrc = notificationIcons.warningIcon;
  } else if (type === "error") {
    notificationClass += " error";
    iconSrc = notificationIcons.errorIcon;
  }

  return (
    <div className={notificationClass}>
      <img src={iconSrc} alt={type} />
      <span>{message}</span>
      <button className="close-btn" onClick={onClose}>×</button>
    </div>
  );
};

export default Notification;
