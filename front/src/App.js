import Notification from "./components/Notification";
import RubiksCube from "./components/RubicsCube";


import { useState, useRef } from "react";
import axios from "axios";
import "./App.css";

const COLORS = {
  O: "orange",
  B: "blue",
  G: "green",
  R: "red",
  W: "white",
  Y: "yellow",
};

// const stepImages = {
//   "F": "/images/F.png",
//   "F'": "/images/F_prime.png",
//   "R": "/images/R.png",
//   "R'": "/images/R_prime.png",
//   "U": "/images/U.png",
//   "U'": "/images/U_prime.png",
//   "U2": "/images/U2.png",
//   // ... остальные шаги
// };

function App() {
  const inputRefs = useRef(Array(12).fill(null));

  const [notifications, setNotifications] = useState([]);

  const addNotification = (message, type = 'success') => {
    setNotifications((prevNotifications) => [
      ...prevNotifications,
      { id: Date.now(), message, type },
    ]);
  };

  const removeNotification = (id) => {
    setNotifications((prevNotifications) =>
      prevNotifications.filter((notification) => notification.id !== id)
    );
  };

  const [selectedImages, setSelectedImages] = useState(Array(12).fill(null));
  const [colorData, setColorData] = useState([]);

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
      newImages[index] = e.target.result;
      setSelectedImages(newImages);
    };
    reader.readAsDataURL(file);
  };

  const handleChange = (e, index) => {
    if (e.target.files && e.target.files.length > 0) {
      setColorData([]);
      processFiles(Array.from(e.target.files), index);
      e.target.value = "";
    }
  };

  const onButtonClick = (index) => {
    setColorData([]);
    inputRefs.current[index].click();
  };

  const removeImage = (index) => {
    setColorData([]);
    const newImages = [...selectedImages];
    newImages[index] = null;
    setSelectedImages(newImages);
  };

  const handleUpload = async () => {
    if (colorData.length !== 6) {
      if (selectedImages.every((img) => img === null)) {
        addNotification("There are no images to upload", "error");
        return;
      }

      // Zone order: Front, Back, Up, Down, Left, Right
      const zoneOrder = [5, 7, 1, 9, 4, 6];

      const imagesToUpload = zoneOrder.map((zoneIndex) => selectedImages[zoneIndex]);

      const formData = new FormData();

      imagesToUpload.forEach((image, index) => {
        if (image) {
          const blob = dataURLToBlob(image);
          formData.append("images", blob, `image_${index}.png`);
        }
      });

      try {
        await axios.post("http://localhost:8014/upload-images", formData, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        });
        addNotification("Images have been uploaded successfully!", "success");
      } catch (error) {
        addNotification("Failed to upload images!", "error");
      }
    }
  };

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

  const [isDetecting, setIsDetecting] = useState(false);
  const [buttonText, setButtonText] = useState("Next");

  const handleDetect = async () => {
    if (colorData.length !== 6) {
      setIsDetecting(true);
      setButtonText("Processing...");

      try {
        const response = await axios.get("http://localhost:8014/detect-colors");
        if (response.status === 200) {
          setColorData(response.data);
          addNotification("Colors have been detected successfully!", "success");
        } else {
          throw new Error("Unexpected response status");
        }
      } catch (error) {
        addNotification("Failed to detect colors!", "error");
      } finally {
        setIsDetecting(false);
        setButtonText("Next");
      }
    }
  };

  const [solution, setSolution] = useState([]);

  const handleUpdateAndSolve = async () => {

    try {
      const requestBody = {
        _front: colorData[0],
        _back: colorData[1],
        _up: colorData[2],
        _down: colorData[3],
        _left: colorData[4],
        _right: colorData[5]
      };

      const responseUpdate = await fetch("http://localhost:8014/update-colors", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(requestBody),
      });

      if (!responseUpdate.ok) throw new Error("Upload colors error");

      const responseSolve = await fetch("http://localhost:8014/solve");
      if (!responseSolve.ok) throw new Error("Trasmit solution error");
      else {
        const solutionData = await responseSolve.json();
        setSolution(solutionData);
        addNotification("Solution have been found!", "success");
      }
    } catch (error) {
      addNotification("Failed to solve!", "error");
    }
  };

  const [isSolveByPhotoMode, setIsSolveByPhotoMode] = useState(false);
  const [isChaosMode, setIsChaosMode] = useState(false);
  const areAllImagesSelected = selectedImages.filter(image => image !== null).length === 6;
  const [isImagePage, setIsImagePage] = useState(false);
  const [isColorPage, setIsColorPage] = useState(false);
  const [isSolvePage, setIsSolvePage] = useState(false);

  return (
    <div className="app-container">
      <div className="main-container">
        <div
          className={`main-content ${isSolveByPhotoMode ? 'solve-by-photo-mode' : ''} ${isChaosMode ? 'chaos-mode' : ''}`}
        >
          {(!isSolveByPhotoMode && !isChaosMode) && (
            <>
              <button
                className="solve-by-photo-btn"
                onClick={() => {
                  setIsSolveByPhotoMode(true);
                  setIsImagePage(true);
                  setIsColorPage(false);
                  setIsSolvePage(false);
                }}
              >
                Solve by photo
              </button>

              <button
                className="chaos-mode-btn"
                onClick={() => setIsChaosMode(true)}
              >
                Chaos Mode
              </button>
            </>
          )}

          {isSolveByPhotoMode && (
            isImagePage && (
              <div className="page-container">
                <div className="side-element">
                  <button
                    className="back-btn"
                    onClick={() => {
                      setIsSolveByPhotoMode(false);
                      setIsImagePage(false);
                      setIsColorPage(false);
                      setIsSolvePage(false);
                      setSelectedImages(Array(12).fill(null));
                      setColorData([]);
                    }}
                  >
                    Back
                  </button>
                </div>

                <div className="central-element">
                  <div className="image-grid">
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
                          className="grid-area"
                          onClick={() => onButtonClick(index)}
                          style={{
                            visibility: [1, 4, 5, 6, 7, 9].includes(index) ? "visible" : "hidden",
                            pointerEvents: [1, 4, 5, 6, 7, 9].includes(index) ? "auto" : "none",
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
                </div>

                <div className="side-element">
                  <button
                    className="next-btn"
                    onClick={async () => {
                      await handleUpload();
                      await handleDetect();
                      setIsImagePage(false);
                      setIsColorPage(true);
                      setIsSolvePage(false);
                    }}
                    disabled={!areAllImagesSelected || isDetecting}
                  >
                    {buttonText}
                  </button>
                </div>
              </div>
            ))
          }

          {isSolveByPhotoMode && (
            isColorPage && (
              <div className="page-container">

                <div className="side-element">
                  <button
                    className="back-btn"
                    onClick={() => {
                      setIsImagePage(true);
                      setIsColorPage(false);
                      setIsSolvePage(false);
                    }}
                  >
                    Back
                  </button>
                </div>

                <div className="central-element">
                  {colorData.length === 6 && (
                    <div className="color-grid">
                      {[...Array(12)].map((_, index) => {
                        const specialIndices = [5, 7, 1, 9, 4, 6];
                        const colorDataIndex = specialIndices.indexOf(index);
                        const row = colorData[colorDataIndex];
                        return specialIndices.includes(index) ? (
                          <div key={index} className="grid-element">
                            {row.map((color, i) => (
                              <div
                                key={i}
                                className="element-cell"
                                style={{ backgroundColor: COLORS[color] || "gray" }}
                                onClick={() => {
                                  const colorKeys = Object.keys(COLORS);
                                  const currentIndex = colorKeys.indexOf(color);
                                  const nextIndex = (currentIndex + 1) % colorKeys.length;
                                  const nextColor = colorKeys[nextIndex];

                                  const newColorData = [...colorData];
                                  newColorData[colorDataIndex][i] = nextColor;
                                  setColorData(newColorData);
                                }}
                              />
                            ))}
                          </div>
                        ) : (
                          <div key={index} className="element-cell.disabled" />
                        );
                      })}
                    </div>
                  )}

                </div>

                <div className="side-element">
                  <button
                    className="next-btn"
                    onClick={async () => {
                      await handleUpdateAndSolve();
                      setIsImagePage(false);
                      setIsColorPage(false);
                      setIsSolvePage(true);
                    }}
                  >
                    {buttonText}
                  </button>
                </div>

              </div>
            ))
          }

          {isSolveByPhotoMode && (
            isSolvePage && (
              <div className="page-container">

                <div className="side-element">
                  <button
                    className="back-btn"
                    onClick={() => {
                      setIsImagePage(false);
                      setIsColorPage(true);
                      setIsSolvePage(false);
                    }}
                  >
                    Back
                  </button>
                </div>

                <div className="solve-page-central-element">
                  <h2>Solution:</h2>
                  {solution.length > 0 ? (
                    <ul className="solution-list">
                      {solution.map((step, index) => (
                        <li key={index} className="solution-step">{step}</li>
                      ))}
                    </ul>
                  ) : (
                    <p>Waiting for solution...</p>
                  )}
                </div>

                <div className="side-element">
                  <button
                    className="next-btn"
                    onClick={async () => {
                      await handleUpload();
                      await handleDetect();
                      setIsSolveByPhotoMode(false);
                      setIsImagePage(false);
                      setIsColorPage(false);
                      setIsSolvePage(false);
                      setSelectedImages(Array(12).fill(null));
                      setColorData([]);
                    }}
                  >
                    {buttonText}
                  </button>
                </div>

              </div>
            ))
          }

          {isChaosMode && (
            <div className="page-container">
              <div className="chaos-side-element">
                <button className="back-btn" onClick={() => setIsChaosMode(false)}>
                  Back
                </button>
              </div>

              <div className="chaos-central-element">
                <RubiksCube />
              </div>
            </div>
          )}

        </div>
      </div>

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
