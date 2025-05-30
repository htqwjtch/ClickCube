import "./RubicsCube.css";

import React, { useRef, useState, useEffect, useCallback } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";

const CUBE_SIZE = 1;
const GAP_RATIO = 0.02;
const TOTAL_SIZE = CUBE_SIZE * (1 + GAP_RATIO);

const faceColors = {
  front: "#ffa40d",
  back: "#ea0600",
  right: "#180c8a",
  left: "#07a42e",
  up: "#fff144",
  down: "#dedede",
  inner: "#888888"
};

const CubePiece = ({ position, colors }) => {
  return (
    <mesh position={position}>
      <boxGeometry args={[CUBE_SIZE, CUBE_SIZE, CUBE_SIZE]} />
      <meshStandardMaterial attach="material-0" color={colors.right} />
      <meshStandardMaterial attach="material-1" color={colors.left} />
      <meshStandardMaterial attach="material-2" color={colors.up} />
      <meshStandardMaterial attach="material-3" color={colors.down} />
      <meshStandardMaterial attach="material-4" color={colors.front} />
      <meshStandardMaterial attach="material-5" color={colors.back} />
    </mesh>
  );
};

const RotatingGroup = ({
  cubes,
  rotatingFace,
  rotationDirection,
  onRotationComplete
}) => {
  const groupRef = useRef();
  const [rotationProgress, setRotationProgress] = useState(0);

  useFrame(() => {
    if (!rotatingFace || !groupRef.current) return;

    const speed = 0.05;
    const targetAngle = Math.PI / 2;
    const direction = rotationDirection === "ccw" ? -1 : 1;
    const newProgress = Math.min(rotationProgress + speed, targetAngle);

    setRotationProgress(newProgress);

    switch (rotatingFace) {
      case "front":
        groupRef.current.rotation.z = -newProgress * direction;
        break;
      case "back":
        groupRef.current.rotation.z = newProgress * direction;
        break;
      case "right":
        groupRef.current.rotation.x = -newProgress * direction;
        break;
      case "left":
        groupRef.current.rotation.x = newProgress * direction;
        break;
      case "up":
        groupRef.current.rotation.y = -newProgress * direction;
        break;
      case "down":
        groupRef.current.rotation.y = newProgress * direction;
        break;
      default:
        break;
    }

    if (newProgress >= targetAngle) {
      onRotationComplete();
      setRotationProgress(0);
      groupRef.current.rotation.set(0, 0, 0);
    }
  });

  return (
    <group ref={groupRef}>
      {cubes.map((cube, index) => {
        const isPartOfFace =
          (rotatingFace === "front" && cube.position[2] === 1) ||
          (rotatingFace === "back" && cube.position[2] === -1) ||
          (rotatingFace === "right" && cube.position[0] === 1) ||
          (rotatingFace === "left" && cube.position[0] === -1) ||
          (rotatingFace === "up" && cube.position[1] === 1) ||
          (rotatingFace === "down" && cube.position[1] === -1);

        return isPartOfFace ? (
          <CubePiece
            key={`rotating-${index}`}
            position={cube.position.map(coord => coord * TOTAL_SIZE)}
            colors={cube.colors}
          />
        ) : null;
      })}
    </group>
  );
};

const RubiksCube = () => {
  const [cubes, setCubes] = useState([]);
  const [rotatingFace, setRotatingFace] = useState(null);
  const [rotationDirection, setRotationDirection] = useState("cw");
  const [isRotating, setIsRotating] = useState(false);
  const [isScrambling, setIsScrambling] = useState(false);
  const scrambleQueue = useRef([]);

  useEffect(() => {
    const newCubes = [];
    for (let x = -1; x <= 1; x++) {
      for (let y = -1; y <= 1; y++) {
        for (let z = -1; z <= 1; z++) {
          if (x === 0 && y === 0 && z === 0) continue;

          const colors = {
            front: z === 1 ? faceColors.front : z === -1 ? faceColors.back : faceColors.inner,
            back: z === -1 ? faceColors.back : z === 1 ? faceColors.front : faceColors.inner,
            right: x === 1 ? faceColors.right : x === -1 ? faceColors.left : faceColors.inner,
            left: x === -1 ? faceColors.left : x === 1 ? faceColors.right : faceColors.inner,
            up: y === 1 ? faceColors.up : y === -1 ? faceColors.down : faceColors.inner,
            down: y === -1 ? faceColors.down : y === 1 ? faceColors.up : faceColors.inner
          };

          if (x === 0) {
            colors.left = faceColors.left;
            colors.right = faceColors.right;
          }
          if (y === 0) {
            colors.up = faceColors.up;
            colors.down = faceColors.down;
          }
          if (z === 0) {
            colors.front = faceColors.front;
            colors.back = faceColors.back;
          }

          newCubes.push({
            position: [x, y, z],
            colors
          });
        }
      }
    }
    setCubes(newCubes);
  }, []);

  const rotateColors = useCallback((colors, face, direction) => {
    const newColors = { ...colors };

    if (face === "front" || face === "back") {
      const temp = newColors.up;
      if (direction === "cw") {
        newColors.up = newColors.left;
        newColors.left = newColors.down;
        newColors.down = newColors.right;
        newColors.right = temp;
      } else {
        newColors.up = newColors.right;
        newColors.right = newColors.down;
        newColors.down = newColors.left;
        newColors.left = temp;
      }
    }
    else if (face === "right" || face === "left") {
      const temp = newColors.up;
      if (direction === "cw") {
        newColors.up = newColors.back;
        newColors.back = newColors.down;
        newColors.down = newColors.front;
        newColors.front = temp;
      } else {
        newColors.up = newColors.front;
        newColors.front = newColors.down;
        newColors.down = newColors.back;
        newColors.back = temp;
      }
    }
    else if (face === "up" || face === "down") {
      const temp = newColors.front;
      if (direction === "cw") {
        newColors.front = newColors.left;
        newColors.left = newColors.back;
        newColors.back = newColors.right;
        newColors.right = temp;
      } else {
        newColors.front = newColors.right;
        newColors.right = newColors.back;
        newColors.back = newColors.left;
        newColors.left = temp;
      }
    }

    return newColors;
  }, []);

  const updateCubePositionsAndColors = useCallback((face, direction) => {
    setCubes(prevCubes => {
      return prevCubes.map(cube => {
        const [x, y, z] = cube.position;
        let newPos = [x, y, z];
        let newColors = { ...cube.colors };

        const shouldRotate =
          (face === "front" && z === 1) ||
          (face === "back" && z === -1) ||
          (face === "right" && x === 1) ||
          (face === "left" && x === -1) ||
          (face === "up" && y === 1) ||
          (face === "down" && y === -1);

        if (shouldRotate) {
          switch (face) {
            case "front":
              newPos = direction === "cw" ? [y, -x, z] : [-y, x, z];
              newColors = rotateColors(cube.colors, "front", direction);
              break;
            case "back":
              newPos = direction === "cw" ? [-y, x, z] : [y, -x, z];
              newColors = rotateColors(cube.colors, "back", direction);
              break;
            case "right":
              newPos = direction === "cw" ? [x, z, -y] : [x, -z, y];
              newColors = rotateColors(cube.colors, "right", direction);
              break;
            case "left":
              newPos = direction === "cw" ? [x, -z, y] : [x, z, -y];
              newColors = rotateColors(cube.colors, "left", direction);
              break;
            case "up":
              newPos = direction === "cw" ? [-z, y, x] : [z, y, -x];
              newColors = rotateColors(cube.colors, "up", direction);
              break;
            case "down":
              newPos = direction === "cw" ? [z, y, -x] : [-z, y, x];
              newColors = rotateColors(cube.colors, "down", direction);
              break;
            default:
              break;
          }
        }

        return { ...cube, position: newPos, colors: newColors };
      });
    });
  }, [rotateColors]);

  const handleRotationComplete = useCallback(() => {
    updateCubePositionsAndColors(rotatingFace, rotationDirection);

    if (scrambleQueue.current.length > 0) {
      const [nextFace, nextDirection] = scrambleQueue.current[0];
      scrambleQueue.current = scrambleQueue.current.slice(1);
      setRotatingFace(nextFace);
      setRotationDirection(nextDirection);
      setIsRotating(true);
    } else {
      setIsRotating(false);
      setIsScrambling(false);
    }
  }, [rotatingFace, rotationDirection, updateCubePositionsAndColors]);

  const rotateFace = useCallback((face, direction) => {
    if (!isRotating && !isScrambling) {
      setRotatingFace(face);
      setRotationDirection(direction);
      setIsRotating(true);
    }
  }, [isRotating, isScrambling]);

  const scrambleCube = useCallback(() => {
    if (isRotating || isScrambling) return;

    const faces = ["front", "back", "right", "left", "up", "down"];
    const directions = ["cw", "ccw"];
    const moves = [];

    for (let i = 0; i < 40; i++) {
      const randomFace = faces[Math.floor(Math.random() * faces.length)];
      const randomDirection = directions[Math.floor(Math.random() * directions.length)];
      moves.push([randomFace, randomDirection]);
    }

    setIsScrambling(true);
    scrambleQueue.current = moves;

    if (moves.length > 0) {
      const [firstFace, firstDirection] = moves[0];
      scrambleQueue.current = moves.slice(1);
      setRotatingFace(firstFace);
      setRotationDirection(firstDirection);
      setIsRotating(true);
    }
  }, [isRotating, isScrambling]);

  return (
    <>
      <Canvas className="canvas">
        <ambientLight intensity={1.2} />
        <directionalLight position={[5, 5, 5]} intensity={0.8} />
        <directionalLight position={[-5, -5, -5]} intensity={0.8} />

        {isRotating && (
          <RotatingGroup
            cubes={cubes}
            rotatingFace={rotatingFace}
            rotationDirection={rotationDirection}
            onRotationComplete={handleRotationComplete}
          />
        )}

        <group>
          {cubes.map((cube, index) => {
            const isPartOfFace = rotatingFace && (
              (rotatingFace === "front" && cube.position[2] === 1) ||
              (rotatingFace === "back" && cube.position[2] === -1) ||
              (rotatingFace === "right" && cube.position[0] === 1) ||
              (rotatingFace === "left" && cube.position[0] === -1) ||
              (rotatingFace === "up" && cube.position[1] === 1) ||
              (rotatingFace === "down" && cube.position[1] === -1)
            );

            return !isPartOfFace || !isRotating ? (
              <CubePiece
                key={`static-${index}`}
                position={cube.position.map(coord => coord * TOTAL_SIZE)}
                colors={cube.colors}
              />
            ) : null;
          })}
        </group>

        <OrbitControls />
      </Canvas>

      <div className="controls-container">
        <button
          onClick={scrambleCube}
          className="scramble-btn"
          disabled={isRotating || isScrambling}
        >
          {isScrambling ? "Scrambling..." : "Scramble"}
        </button>

        {Object.entries({
          front: "#ffa40d",
          back: "#ea0600",
          right: "#180c8a",
          left: "#07a42e",
          up: "#fff144",
          down: "#ffffff"
        }).map(([face, color]) => (
          <div key={face} className="face-row">
            <button
              onClick={() => rotateFace(face, "cw")}
              className="face-btn-cw"
              disabled={isRotating || isScrambling}
            >
              ↻
            </button>
            <div className="face-lbl" style={{ color }}>
              {face.charAt(0).toUpperCase() + face.slice(1)}
            </div>
            <button
              onClick={() => rotateFace(face, "ccw")}
              className="face-btn-ccw"
              disabled={isRotating || isScrambling}
            >
              ↺
            </button>
          </div>
        ))}
      </div>
    </>
  );
};

export default RubiksCube;