import React, { useEffect } from "react";
import "./Notification.css";

const Notification = ({ message, type, onClose }) => {
  useEffect(() => {
    const timer = setTimeout(onClose, 4000); // Убираем уведомление через 4 секунды
    return () => clearTimeout(timer);
  }, [onClose]);

  let notificationClass = "notification";

  // В зависимости от типа уведомления добавляем класс
  if (type === "success") {
    notificationClass += " success";
  } else if (type === "warning") {
    notificationClass += " warning";
  } else if (type === "error") {
    notificationClass += " error";
  }

  return (
    <div className={notificationClass}>
      <span>{message}</span>
      <button className="close-btn" onClick={onClose}>×</button>
    </div>
  );
};

export default Notification;
