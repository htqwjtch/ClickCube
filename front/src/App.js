import { useState, useRef } from "react";
import axios from "axios";
import Notification from "./components/Notification"; // Импортируем компонент уведомлений
import "./App.css";

function App() {
  const [selectedImages, setSelectedImages] = useState(Array(12).fill(null)); // State to hold images for each zone
  const [notifications, setNotifications] = useState([]); // Уведомления

  // Массив рефов для каждого input
  const inputRefs = useRef(Array(12).fill(null));

  // Массив активных зон (индексы зон, которые нужно оставить)
  const activeZones = [1, 4, 5, 6, 7, 9];

  // Функция для добавления уведомлений
  const addNotification = (message, type = 'success') => {
    setNotifications((prevNotifications) => [
      ...prevNotifications,
      { id: Date.now(), message, type },
    ]);
  };

  // Функция для удаления уведомления
  const removeNotification = (id) => {
    setNotifications((prevNotifications) =>
      prevNotifications.filter((notification) => notification.id !== id)
    );
  };

  // Функция обработки файлов
  const processFiles = (files, index) => {
    if (files.length > 1) {
      addNotification("Пожалуйста, загружайте только одно изображение за раз", "warning");
      return;
    }

    const file = files[0];
    if (!file.type.match('image.*')) {
      addNotification("Пожалуйста, загружайте только изображения", "warning");
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const newImages = [...selectedImages];
      newImages[index] = e.target.result; // Assign the image to the specific zone
      setSelectedImages(newImages);
    };
    reader.readAsDataURL(file);
  };

  // Обработчик выбора файла через клик
  const handleChange = (e, index) => {
    if (e.target.files && e.target.files.length > 0) {
      processFiles(Array.from(e.target.files), index);
      e.target.value = ""; // Clear input value to allow re-uploading
    }
  };

  const onButtonClick = (index) => {
    inputRefs.current[index].click(); // Вызываем click для конкретного input
  };

  const removeImage = (index) => {
    const newImages = [...selectedImages];
    newImages[index] = null; // Remove image from the specific zone
    setSelectedImages(newImages);
  };

  // Функция загрузки файлов на сервер
  const handleUpload = async () => {
    if (selectedImages.every((img) => img === null)) {
      addNotification("Нет изображений для загрузки", "error");
      return;
    }

    // Zone order: Front, Back, Up, Down, Left, Right
    const zoneOrder = [5, 7, 1, 9, 4, 6]; // Active zones in the required upload order

    // Rearrange selected images based on the zoneOrder
    const imagesToUpload = zoneOrder.map((zoneIndex) => selectedImages[zoneIndex]);

    const formData = new FormData();

    imagesToUpload.forEach((image, index) => {
      if (image) {
        const blob = dataURLToBlob(image);
        formData.append("images", blob, `image_${index}.png`);
      }
    });

    try {
      await axios.post("http://localhost:8013/upload-images", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      });
      addNotification("Изображения загружены успешно!", "success");
      setSelectedImages(Array(12).fill(null)); // Очищаем загруженные изображения
    } catch (error) {
      console.error("Ошибка загрузки", error);
      addNotification("Ошибка загрузки изображений", "error");
    }
  };

  // Конвертация dataURL в Blob
  const dataURLToBlob = (dataURL) => {
    const byteString = atob(dataURL.split(",")[1]);
    const mimeString = dataURL.split(",")[0].split(":")[1].split(";")[0];

    const arrayBuffer = new ArrayBuffer(byteString.length);
    const uint8Array = new Uint8Array(arrayBuffer);
    for (let i = 0; i < byteString.length; i++) {
      uint8Array[i] = byteString.charCodeAt(i);
    }

    return new Blob([uint8Array], { type: mimeString });
  };

  // Проверка, чтобы кнопка загрузки появлялась только при 6 изображениях
  const isUploadButtonVisible = selectedImages.filter(image => image !== null).length === 6;

  return (
    <div className="app-container">
      <div className="container">
        <div className="main-content">
          <div className="upload-grid">
            {selectedImages.map((image, index) => {
              // Заменяем текст на соответствующие значения для зон
              let zoneLabel = `Зона ${index}`;
              if (index === 1) zoneLabel = "Up";
              if (index === 4) zoneLabel = "Left";
              if (index === 5) zoneLabel = "Front";
              if (index === 6) zoneLabel = "Right";
              if (index === 7) zoneLabel = "Back";
              if (index === 9) zoneLabel = "Down";

              return (
                <div
                  key={index}
                  className="upload-area"
                  onClick={() => onButtonClick(index)} // Только клик по зоне
                  style={{
                    visibility: activeZones.includes(index) ? "visible" : "hidden", // Make non-active zones hidden
                    pointerEvents: activeZones.includes(index) ? "auto" : "none"  // Disable pointer events on non-active zones
                  }}
                >
                  <input
                    type="file"
                    ref={(el) => inputRefs.current[index] = el} // Ссылка на каждый input
                    onChange={(e) => handleChange(e, index)}
                    className="file-input"
                    accept="image/*"
                    multiple={false}
                    id={`file-input-${index}`} // Уникальный id для каждого input
                  />
                  {image ? (
                    <div className="image-preview-container">
                      <img src={image} alt={`preview ${index}`} className="image-preview" />
                      <button
                        className="remove-btn"
                        onClick={(e) => {
                          e.stopPropagation();
                          removeImage(index);
                        }}
                      >
                        ×
                      </button>
                    </div>
                  ) : (
                    <p>{zoneLabel}</p>
                  )}
                </div>
              );
            })}
          </div>

          {/* Кнопка загрузки */}
          {isUploadButtonVisible && (
            <button className="upload-btn" onClick={handleUpload}>
              Загрузить изображения
            </button>
          )}
        </div>
      </div>

      {/* Уведомления */}
      <div className="notifications-container">
        {notifications.map(({ id, message, type }) => (
          <Notification
            key={id}
            message={message}
            type={type}
            onClose={() => removeNotification(id)}
          />
        ))}
      </div>
    </div>
  );
}

export default App;
