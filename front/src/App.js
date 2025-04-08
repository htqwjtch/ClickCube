import Notification from "./components/Notification";
import RubiksCube from "./components/RubicsCube";

import { useState, useRef } from "react";
import axios from "axios";
import "./App.css";

const COLORS = {
  O: "#ffa40d",
  R: "#ea0600",
  B: "#180c8a",
  G: "#07a42e",
  Y: "#fff144",
  W: "#ffffff"
};

const stepImgs = require.context('../public/assets/icons/solution-steps/', false, /\.png$/);

const stepImages = stepImgs.keys().map(stepImgs);

const stepTitles = [
  "Daisy",
  "Down Cross",
  "First Layer",
  "Second Layer",
  "Up Cross",
  "Right Up Cross",
  "Up Corners",
  "Third Layer"
];

const stepDescriptions = [
  "To create a daisy, you need to find all 4 white edges and raise them to the yellow center.",
  "You should look at the flower and pay attention to the color of the rib. Then select any of the edges, align it with the center of the same color and lower it down (make the F2 movement). This should be done alternately with all the edges.\n\nPlease note that the color of the edges in the cross must match the colors of the centers. If you get the wrong cross, you should try again!",
  "Next, you should collect the corners (3 colors on the element), as a result, you should get the whole first layer.\n\nBut first you need to learn a very simple but effective formula called Pif-Paf. It consists of 4 movements R U R' U', and in simple words: up-left-down-right.",
  "To make the second layer, you need to put the edges from the top layer to the middle one in turn. To do this, you need to learn Pif-Paf with your left hand L' U' L U or in other words, with your left hand you should perform up-right-down-left",
  "At this stage, turn the cube yellow up again.\n\nThe sequence of creating the yellow cross is as follows:\nPoint → Floor of the cross (nine o'clock) → Stick → Cross\n\nA whole cross can already be assembled on the cube. But it's not always that lucky. Sometimes it still turns out to be one sad point that needs to be gradually transformed into a cross.\n\nWhatever the shape is, you should do F R U R' U' F'. Simply put, F (Pif-Paf) F'.\n\nPlease note that if there is a \"Floor of the cross\", then you need to hold it for 9 o'clock, so that one arrow looks to the left and the other up. And if it's a stick, then you need to hold it horizontally and only then do the algorithm.\n\nIf there was a dot initially, then it will turn into a half cross, if there was a half cross, then they will turn into a stick, and if a stick, then into a cross.\n\nAttention!\nIf 1 or 3 elements are looking up at the same time and none of the above figures are present, the cube could be disassembled into parts and incorrectly assembled (accidentally or for the purpose of playing a joke). In this case, it is impossible to assemble a cube using formulas. It is strongly recommended to take the cube apart and assemble it by color mechanically.",
  "The resulting cross may be incorrect. we need to make sure that all the colors on the edges of the cross (3 layers) match the centers.\n\nThere can only be three situations:\n1. The edges match next to each other.If these two edges are adjacent, you should put them on the right and behind, and make the formula R U R' U R U2 R' U.\n2. The edges match opposite.If these two edges coincide opposite, you should put them in front and behind, and make the formula R U R' U R U2 R' and the previous situation should be obtained.\n3. The cross is immediately correct or you only need to turn the top layer.",
  "First you need to check how many corners are in place. It's either everything, 1 corner, or none. If all the corners are in place, skip this step and move on to the next one.\n\nThe corners can look yellow in any direction. The main thing is that the orange-green corner stands between the orange and green center, the green-red corner stands between the green and red, etc.\n\nIf there are no such angles, then do the algorithm:\nR U' L' U R' U' L U\n\nAfter executing the algorithm, 1 correct angle should appear. You need to put it on the left and on top (where your left thumb is) and do the same algorithm. If nothing works out, put it back in the upper-left point and repeat.\n\nAttention!\nIf only 2 or 3 elements are in place, the cube was disassembled into parts and incorrectly assembled. From such a state, the cube cannot be assembled according to formulas. You should take the cube apart and assemble it by color mechanically.\n\nWhen all the corners are in place, all that remains is to unfold them in order to fully assemble the cube.",
  "The corners should be turned Pif-Paf.\n\nYou should check again whether all the corners are in place. Intercept the cube so that the corners that need to be collected are from below. You can start from any corner that is not assembled and do bang-bang until the corner is assembled. Then you need to rotate the bottom layer and put the next unrolled corner. Keep doing bang-bang until the 2nd corner comes together and so on.\n\nAttention!\nIf the corner has already been assembled, bang-bang still needs to be completed, and only then proceed to the next one).\n\nWhen all the corners are unfolded, the cube will be assembled!"
];

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
  const [nextText, setButtonText] = useState("Next");

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

  const handleUpdate = async () => {

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

      if (!responseUpdate.ok) throw new Error("Update colors error");
      else {
        addNotification("Colors have been updated successfully!", "success");
      }

    } catch (error) {
      addNotification("Failed to update!", "error");
    }
  };

  const [solution, setSolution] = useState([]);

  const handleSolve = async () => {

    try {
      const responseSolve = await fetch("http://localhost:8014/solve");
      if (!responseSolve.ok) throw new Error("Trasmit solution error");
      const solutionData = await responseSolve.json();

      // Обрабатываем каждый этап решения
      const processedSolution = solutionData.map(step => {
        // Если этап пустой (пустой массив или массив с пустыми строками)
        if (!step || step.length === 0 || (step.length === 1 && step[0].trim() === "")) {
          return ["You're in luck! You can skip this step"];
        }
        return step;
      });

      setSolution(processedSolution);
      addNotification("Solution has been found!", "success");
    } catch (error) {
      addNotification("Failed to solve!", "error");
    }
  };

  const [isSolveByPhotoMode, setIsSolveByPhotoMode] = useState(false);
  const [isChaosMode, setIsChaosMode] = useState(false);
  const areAllImagesSelected = selectedImages.filter(image => image !== null).length === 6;
  const [isImagePage, setIsImagePage] = useState(false);
  const [isHelpModalOpen, setIsHelpModalOpen] = useState(false);
  const [isColorPage, setIsColorPage] = useState(false);
  const [isSolvePage, setIsSolvePage] = useState(false);
  const [isSingMasterNotation, setIsSingMasterNotation] = useState(false);

  const getNotationLabel = (index) => {
    const labels = [
      "F", "F'", "B", "B'",
      "R", "R'", "L", "L'",
      "U", "U'", "D", "D'",
      "X", "X'", "Y", "Y'"
    ];
    return labels[index - 1] || `Move ${index}`;
  };

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

                <div className="central-container">
                  <div className="title-container">
                    <div className="empty-container">
                    </div>
                    <div className="page-title">
                      Upload photos of Rubik's Cube faces
                    </div>
                    <div className="help-container">
                      <button
                        className="help-btn"
                        onClick={() => setIsHelpModalOpen(true)}
                      >
                        ?
                      </button>
                    </div>
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
                  </div>

                  {isHelpModalOpen && (
                    <div className="modal-overlay" onClick={() => setIsHelpModalOpen(false)}>
                      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
                        <h2>Requirements for photos of Rubik's Cube faces</h2>
                        <div className="instructions">
                          <p className="intro">If you thought uploading photos will be easy, then sit down)</p>
                          <p className="section">1. Photo format:</p>
                          <p className="bullet">• Photos should be square (like the face of a cube).</p>
                          <p className="bullet">• The face of the cube should occupy almost the entire area of the photo (as large as possible, without unnecessary background).</p>

                          <p className="section">2. The colors of the centers (a guideline, but not necessarily):</p>
                          <p className="bullet">• The front face (Front) is the orange center.</p>
                          <p className="bullet">• The back face (Back) is the red center.</p>
                          <p className="bullet">• The upper face (Up) is the yellow center.</p>
                          <p className="bullet">• The lower face (Down) is the white center.</p>
                          <p className="bullet">• The right face (Right) is the blue center.</p>
                          <p className="bullet">• The left face (Left) is the green center.</p>
                          <p className="note">If you have a cube with a different color scheme, you can load it as it is, but it is important to keep the correct orientation.</p>

                          <p className="section">3. Orientation of the faces in the photo</p>
                          <p className="explanation">The main thing is to rotate the face correctly before photographing (for example, the front face with orange color):</p>
                          <p className="bullet">• For the side faces (orange, blue, red, green centers), the upper face should have a yellow center, the lower face should have a white center.</p>
                          <p className="bullet">• For the upper and lower faces (yellow and white centers, respectively), the left and right faces should have green and blue centers, respectively.</p>
                          <p className="bullet">• For the upper face, the top face should be the back face (red), and the bottom face should be the front face (orange). For the lower face, the opposite is true.</p>
                        </div>
                        <button
                          className="close-modal-btn"
                          onClick={() => setIsHelpModalOpen(false)}
                        >
                          ×
                        </button>
                      </div>
                    </div>
                  )}
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
                    {nextText}
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

                <div className="central-container">
                  <div className="title-container">
                    <div className="page-title">
                      Please edit detected colors by clicking, if necessary
                    </div>
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
                </div>

                <div className="side-element">
                  <button
                    className="next-btn"
                    onClick={async () => {
                      await handleUpdate();
                      await handleSolve();
                      setIsImagePage(false);
                      setIsColorPage(false);
                      setIsSolvePage(true);
                    }}
                  >
                    {nextText}
                  </button>
                </div>

              </div>
            ))
          }

          {isSolveByPhotoMode && isSolvePage && (
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
                {/* Заменяем кнопку на выпадающий элемент */}
                <div className={`dropdown-container ${isSingMasterNotation ? 'active' : ''}`}>
                  <div
                    className="dropdown-header"
                    onClick={() => setIsSingMasterNotation(!isSingMasterNotation)}
                  >
                    Singmaster Notation
                    <span className="dropdown-arrow">
                      {isSingMasterNotation ? '▲' : '▼'}
                    </span>
                  </div>

                  <div className="dropdown-content">
                    {isSingMasterNotation && (
                      <div className="notation-table-container">
                        <table className="notation-table">
                          <tbody>
                            {[...Array(4)].map((_, rowIndex) => (
                              <tr key={rowIndex}>
                                {[...Array(4)].map((_, colIndex) => {
                                  const index = rowIndex * 4 + colIndex + 1;
                                  return (
                                    <td key={colIndex}>
                                      <img
                                        src={`/assets/icons/notation/${index}.gif`}
                                        alt={`Notation ${index}`}
                                        className="notation-gif"
                                        onError={(e) => {
                                          e.target.src = '/assets/icons/notation/default.gif'; // fallback если изображение не найдено
                                        }}
                                      />
                                      <div className="notation-label">{getNotationLabel(index)}</div>
                                    </td>
                                  );
                                })}
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    )}
                  </div>
                </div>

                {solution.length > 0 ? (
                  <ul className="solution-list">
                    {solution.map((step, index) => (
                      <div key={index} className="solution-step">
                        <img
                          src={stepImages[index]}
                          alt={`Step ${index + 1}`}
                          style={{
                            width: "100px",
                            height: "100px",
                            padding: "8px",
                            objectFit: "contain"
                          }}
                        />
                        <div className="step-info">
                          <div className="step-title">
                            {stepTitles[index]}
                          </div>
                          <div className="step-notation">
                            {step}
                          </div>
                          <div className="step-desc">
                            {stepDescriptions[index]}
                          </div>
                        </div>
                      </div>
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
                  {nextText}
                </button>
              </div>
            </div>
          )}

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

              <div className="chaos-side-element">
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
