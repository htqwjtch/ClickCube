import { useState, useRef, useCallback } from "react";
import axios from "axios";
import Notification from "./components/Notification"; // Импортируем компонент уведомлений
import "./App.css";

function App() {
  const [dragActive, setDragActive] = useState(false);
  const [selectedImages, setSelectedImages] = useState([]);
  const [notifications, setNotifications] = useState([]); // Состояние для уведомлений
  const inputRef = useRef(null);

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
  const processFiles = useCallback((files) => {
    if (selectedImages.length >= 6) {
      addNotification("Максимум 6 изображений", "warning");
      return;
    }

    files.forEach((file) => {
      if (!file.type.match('image.*')) {
        addNotification("Пожалуйста, загружайте только изображения", "warning");
        return;
      }

      const reader = new FileReader();
      reader.onload = (e) => {
        setSelectedImages(prev => [...prev, e.target.result]);
      };
      reader.readAsDataURL(file);
    });
  }, [selectedImages]);

  // Обработчики drag and drop
  const handleDrag = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }
  }, []);

  const handleDrop = useCallback((e) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const files = Array.from(e.dataTransfer.files);

      if (selectedImages.length + files.length > 6) {
        addNotification("Можно загрузить максимум 6 изображений.", "warning");
        return;
      }

      processFiles(files);
    }
  }, [selectedImages, processFiles]);

  // Обработчик выбора файла через клик
  const handleChange = (e) => {
    if (e.target.files && e.target.files.length > 0) {
      processFiles(Array.from(e.target.files));
      e.target.value = "";
    }
  };

  const onButtonClick = () => {
    inputRef.current.click();
  };

  const removeImage = (index) => {
    setSelectedImages(prev => prev.filter((_, i) => i !== index));
  };

  // Функция загрузки файлов на сервер
  const handleUpload = async () => {
    if (selectedImages.length === 0) {
      addNotification("Нет изображений для загрузки", "error");
      return;
    }

    const formData = new FormData();

    // Преобразуем dataURL обратно в Blob (файл)
    selectedImages.forEach((image, index) => {
      const blob = dataURLToBlob(image);
      formData.append("images", blob, `image_${index}.png`);
    });

    try {
      await axios.post("http://localhost:8013/upload-images", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      });
      addNotification("Изображения загружены успешно!", "success");
      setSelectedImages([]); // Очищаем загруженные изображения
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

  return (
    <div className="app-container">
      <div className="container">
        <div className="main-content">
          {/* Область загрузки */}
          <div
            className={`upload-area ${dragActive ? "drag-active" : ""}`}
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
            onClick={onButtonClick}
          >
            <input
              type="file"
              ref={inputRef}
              onChange={handleChange}
              className="file-input"
              accept="image/*"
              multiple={false}
            />
            <p>Кликните или перетащите изображение</p>
            <p>Выбрано: {selectedImages.length}/6</p>
          </div>

          {/* Превью изображений */}
          <div className="preview-container">
            {selectedImages.map((img, index) => (
              <div key={index} className="image-preview">
                <img src={img} alt={`preview ${index}`} />
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
            ))}
          </div>

          {/* Кнопка загрузки */}
          {selectedImages.length === 6 && (
            <button className="btn" onClick={handleUpload}>
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
