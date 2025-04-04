import React, { useRef, useState, useEffect, useCallback } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";

const CUBE_SIZE = 1;
const GAP_RATIO = 0.02;
const TOTAL_SIZE = CUBE_SIZE * (1 + GAP_RATIO);

const faceColors = {
  front: "orange",
  back: "red",
  right: "blue",
  left: "green",
  top: "yellow",
  bottom: "white",
  inner: "#888888"
};

const CubePiece = ({ position, colors }) => {
  return (
    <mesh position={position}>
      <boxGeometry args={[CUBE_SIZE, CUBE_SIZE, CUBE_SIZE]} />
      <meshStandardMaterial 
        attach="material-0" 
        color={colors.right} 
        emissive={colors.right}
        emissiveIntensity={0.3}
      />
      <meshStandardMaterial 
        attach="material-1" 
        color={colors.left} 
        emissive={colors.left}
        emissiveIntensity={0.3}
      />
      <meshStandardMaterial 
        attach="material-2" 
        color={colors.top} 
        emissive={colors.top}
        emissiveIntensity={0.3}
      />
      <meshStandardMaterial 
        attach="material-3" 
        color={colors.bottom} 
        emissive={colors.bottom}
        emissiveIntensity={0.3}
      />
      <meshStandardMaterial 
        attach="material-4" 
        color={colors.front} 
        emissive={colors.front}
        emissiveIntensity={0.3}
      />
      <meshStandardMaterial 
        attach="material-5" 
        color={colors.back} 
        emissive={colors.back}
        emissiveIntensity={0.3}
      />
    </mesh>
  );
};

const RotatingGroup = ({ 
  cubes, 
  rotatingFace, 
  onRotationComplete 
}) => {
  const groupRef = useRef();
  const [rotationProgress, setRotationProgress] = useState(0);

  useFrame(() => {
    if (!rotatingFace || !groupRef.current) return;
    
    const speed = 0.05;
    const targetAngle = Math.PI / 2;
    const newProgress = Math.min(rotationProgress + speed, targetAngle);
    
    setRotationProgress(newProgress);

    switch (rotatingFace) {
      case "front":
        groupRef.current.rotation.z = -newProgress;
        break;
      case "back":
        groupRef.current.rotation.z = newProgress;
        break;
      case "right":
        groupRef.current.rotation.x = -newProgress;
        break;
      case "left":
        groupRef.current.rotation.x = newProgress;
        break;
      case "top":
        groupRef.current.rotation.y = -newProgress;
        break;
      case "bottom":
        groupRef.current.rotation.y = newProgress;
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
          (rotatingFace === "top" && cube.position[1] === 1) ||
          (rotatingFace === "bottom" && cube.position[1] === -1);

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
  const [isRotating, setIsRotating] = useState(false);

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
            top: y === 1 ? faceColors.top : y === -1 ? faceColors.bottom : faceColors.inner,
            bottom: y === -1 ? faceColors.bottom : y === 1 ? faceColors.top : faceColors.inner
          };

          if (x === 0) {
            colors.left = faceColors.left;
            colors.right = faceColors.right;
          }
          if (y === 0) {
            colors.top = faceColors.top;
            colors.bottom = faceColors.bottom;
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
      const temp = newColors.top;
      if (direction === "cw") {
        newColors.top = newColors.right;
        newColors.right = newColors.bottom;
        newColors.bottom = newColors.left;
        newColors.left = temp;
      } else {
        newColors.top = newColors.left;
        newColors.left = newColors.bottom;
        newColors.bottom = newColors.right;
        newColors.right = temp;
      }
    } 
    else if (face === "right" || face === "left") {
      const temp = newColors.top;
      if (direction === "cw") {
        newColors.top = newColors.front;
        newColors.front = newColors.bottom;
        newColors.bottom = newColors.back;
        newColors.back = temp;
      } else {
        newColors.top = newColors.back;
        newColors.back = newColors.bottom;
        newColors.bottom = newColors.front;
        newColors.front = temp;
      }
    }
    else if (face === "top" || face === "bottom") {
      const temp = newColors.front;
      if (direction === "cw") {
        newColors.front = newColors.right;
        newColors.right = newColors.back;
        newColors.back = newColors.left;
        newColors.left = temp;
      } else {
        newColors.front = newColors.left;
        newColors.left = newColors.back;
        newColors.back = newColors.right;
        newColors.right = temp;
      }
    }

    return newColors;
  }, []);

  const updateCubePositionsAndColors = useCallback((face) => {
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
          (face === "top" && y === 1) ||
          (face === "bottom" && y === -1);

        if (shouldRotate) {
          switch (face) {
            case "front":
              newPos = [y, -x, z];
              newColors = rotateColors(cube.colors, "front", "cw");
              break;
            case "back":
              newPos = [-y, x, z];
              newColors = rotateColors(cube.colors, "back", "cw");
              break;
            case "right":
              newPos = [x, z, -y];
              newColors = rotateColors(cube.colors, "right", "cw");
              break;
            case "left":
              newPos = [x, -z, y];
              newColors = rotateColors(cube.colors, "left", "cw");
              break;
            case "top":
              newPos = [-z, y, x];
              newColors = rotateColors(cube.colors, "top", "cw");
              break;
            case "bottom":
              newPos = [z, y, -x];
              newColors = rotateColors(cube.colors, "bottom", "cw");
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
    updateCubePositionsAndColors(rotatingFace);
    setRotatingFace(null);
    setIsRotating(false);
  }, [rotatingFace, updateCubePositionsAndColors]);

  const rotateFace = useCallback((face) => {
    if (!isRotating) {
      setRotatingFace(face);
      setIsRotating(true);
    }
  }, [isRotating]);

  return (
    <>
      <Canvas style={{ width: "100vw", height: "100vh" }}>
        <ambientLight intensity={1.2} />
        <directionalLight 
          position={[5, 5, 5]} 
          intensity={0.8} 
          castShadow={false}
        />
        <directionalLight 
          position={[-5, -5, -5]} 
          intensity={0.8} 
          castShadow={false}
        />
        
        {isRotating && (
          <RotatingGroup
            cubes={cubes}
            rotatingFace={rotatingFace}
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
              (rotatingFace === "top" && cube.position[1] === 1) ||
              (rotatingFace === "bottom" && cube.position[1] === -1)
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

      <div style={{ 
        position: "relative", 
        display: 'flex',
        gap: '8px'
      }}>
        <button onClick={() => rotateFace("front")}>Front (Orange)</button>
        <button onClick={() => rotateFace("back")}>Back (Red)</button>
        <button onClick={() => rotateFace("right")}>Right (Blue)</button>
        <button onClick={() => rotateFace("left")}>Left (Green)</button>
        <button onClick={() => rotateFace("top")}>Top (Yellow)</button>
        <button onClick={() => rotateFace("bottom")}>Bottom (White)</button>
      </div>
    </>
  );
};

export default RubiksCube;