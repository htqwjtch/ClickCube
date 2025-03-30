import { useState, useRef } from "react";
import axios from "axios";
import Notification from "./components/Notification"; // Импортируем компонент уведомлений
import "./App.css";

function App() {
  const [selectedImages, setSelectedImages] = useState(Array(12).fill(null)); // State to hold images for each zone
  const [notifications, setNotifications] = useState([]); // Уведомления
  const [isSolveByPhotoMode, setIsSolveByPhotoMode] = useState(false); // Tracks if assembly mode is active
  const [isChaosMode, setIsChaosMode] = useState(false); // Tracks if chaos mode is active

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
      addNotification("Please upload only one image", "warning");
      return;
    }

    const file = files[0];
    if (!file.type.match('image.*')) {
      addNotification("Please upload only images", "warning");
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
      addNotification("There are no images to upload", "error");
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
      addNotification("Images have been uploaded successfully!", "success");
      setSelectedImages(Array(12).fill(null)); // Очищаем загруженные изображения
    } catch (error) {
      console.error("Failed to upload images!", error);
      addNotification("Failed to upload images!", "error");
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
        <div
          className={`main-content ${isSolveByPhotoMode ? 'solve-by-photo-mode' : ''} ${isChaosMode ? 'chaos-mode' : ''}`}
        >
          {/* Main Button to start assembly */}
          {(!isSolveByPhotoMode && !isChaosMode) && (
            <>
              <button
                className="solve-by-photo-btn"
                onClick={() => setIsSolveByPhotoMode(true)}
              >
                Solve by photo
              </button>

              {/* New Chaos Mode Button */}
              <button
                className="chaos-mode-btn"
                onClick={() => setIsChaosMode(true)}
              >
                Chaos Mode
              </button>
            </>
          )}

          {/* Go back button */}
          {isSolveByPhotoMode && (
            <div className="back-btn-container">
              <button
                className="back-btn"
                onClick={() => setIsSolveByPhotoMode(false)}
              >
                Back
              </button>
            </div>
          )}

          {/* Go back button */}
          {isChaosMode && (
            <div className="back-btn-container">
              <button
                className="back-btn"
                onClick={() => setIsChaosMode(false)}
              >
                Back
              </button>
            </div>
          )}

          {/* Zones and upload button only show in assembly mode */}
          {isSolveByPhotoMode && (
            <div className="click-container">
              <div className="upload-grid">
                {selectedImages.map((image, index) => {
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
                      onClick={() => onButtonClick(index)}
                      style={{
                        visibility: activeZones.includes(index) ? "visible" : "hidden",
                        pointerEvents: activeZones.includes(index) ? "auto" : "none",
                      }}
                    >
                      <input
                        type="file"
                        ref={(el) => (inputRefs.current[index] = el)}
                        onChange={(e) => handleChange(e, index)}
                        className="file-input"
                        accept="image/*"
                        multiple={false}
                        id={`file-input-${index}`}
                      />
                      {image ? (
                        <div className="image-preview-container">
                          <img src={image} alt={`preview ${index}`} className="image-preview" />
                          <button className="remove-btn"
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

              {/* Upload button only visible when 6 images are selected */}
              {isUploadButtonVisible && (
                <div className="detect-colors-btn-container">
                  <button className="detect-colors-btn" onClick={handleUpload}>
                    Detect colors
                  </button>
                </div>
              )}
            </div>
          )}

        </div>
      </div>

      {/* Notifications */}
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
    </div >
  );

}

export default App;
