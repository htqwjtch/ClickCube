import React, { useEffect } from "react";
import "./Notification.css";
import successIcon from "../assets/icons/notifications/success.png";  // Добавляем иконки
import warningIcon from "../assets/icons/notifications/warning.png";
import errorIcon from "../assets/icons/notifications/error.png";

const Notification = ({ message, type, onClose }) => {
  useEffect(() => {
    const timer = setTimeout(onClose, 4000);
    return () => clearTimeout(timer);
  }, [onClose]);

  let notificationClass = "notification";
  let iconSrc = "";

  if (type === "success") {
    notificationClass += " success";
    iconSrc = successIcon;
  } else if (type === "warning") {
    notificationClass += " warning";
    iconSrc = warningIcon;
  } else if (type === "error") {
    notificationClass += " error";
    iconSrc = errorIcon;
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
