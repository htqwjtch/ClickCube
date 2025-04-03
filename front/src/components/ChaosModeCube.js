import React, { useRef, useState, useEffect } from 'react';
import * as THREE from 'three';

const RubikCube = () => {
  const containerRef = useRef();
  const cubeRef = useRef();
  const [rotation, setRotation] = useState({ x: 0, y: 0, z: 0 });

  // Цвета для каждой стороны кубика Рубика
  const colors = {
    front: 0xff0000,  // Красный
    back: 0x00ff00,   // Зеленый
    left: 0x0000ff,   // Синий
    right: 0xffff00,  // Желтый
    up: 0xffffff,     // Белый
    down: 0x000000    // Черный
  };

  useEffect(() => {
    const width = containerRef.current.clientWidth;
    const height = containerRef.current.clientHeight;
  
    // Создаем сцену и камеру
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    const renderer = new THREE.WebGLRenderer();
    renderer.setSize(width, height);
    containerRef.current.appendChild(renderer.domElement);
  
    const cube = new THREE.Group();
  
    // Геометрия для одной маленькой плитки
    const tileGeometry = new THREE.BoxGeometry(1, 1, 0.1);
  
    // Создаем плитки для каждой стороны
    const createTile = (color, x, y, z) => {
      const material = new THREE.MeshBasicMaterial({ color });
      const tile = new THREE.Mesh(tileGeometry, material);
      tile.position.set(x, y, z);
      return tile;
    };
  
    // Front side (красный)
    const frontSide = [
      createTile(colors.front, 0, 0, 1), createTile(colors.front, 1, 0, 1), createTile(colors.front, 2, 0, 1),
      createTile(colors.front, 0, 1, 1), createTile(colors.front, 1, 1, 1), createTile(colors.front, 2, 1, 1),
      createTile(colors.front, 0, 2, 1), createTile(colors.front, 1, 2, 1), createTile(colors.front, 2, 2, 1)
    ];
  
    // Back side (зеленый)
    const backSide = [
      createTile(colors.back, 0, 0, -1), createTile(colors.back, 1, 0, -1), createTile(colors.back, 2, 0, -1),
      createTile(colors.back, 0, 1, -1), createTile(colors.back, 1, 1, -1), createTile(colors.back, 2, 1, -1),
      createTile(colors.back, 0, 2, -1), createTile(colors.back, 1, 2, -1), createTile(colors.back, 2, 2, -1)
    ];
  
    // Left side (синий)
    const leftSide = [
      createTile(colors.left, -1, 0, 0), createTile(colors.left, -1, 1, 0), createTile(colors.left, -1, 2, 0),
      createTile(colors.left, -2, 0, 0), createTile(colors.left, -2, 1, 0), createTile(colors.left, -2, 2, 0),
      createTile(colors.left, -3, 0, 0), createTile(colors.left, -3, 1, 0), createTile(colors.left, -3, 2, 0)
    ];
  
    // Right side (желтый)
    const rightSide = [
      createTile(colors.right, 1, 0, 0), createTile(colors.right, 1, 1, 0), createTile(colors.right, 1, 2, 0),
      createTile(colors.right, 2, 0, 0), createTile(colors.right, 2, 1, 0), createTile(colors.right, 2, 2, 0),
      createTile(colors.right, 3, 0, 0), createTile(colors.right, 3, 1, 0), createTile(colors.right, 3, 2, 0)
    ];
  
    // Up side (белый)
    const upSide = [
      createTile(colors.up, 0, 1, 1), createTile(colors.up, 1, 1, 1), createTile(colors.up, 2, 1, 1),
      createTile(colors.up, 0, 2, 1), createTile(colors.up, 1, 2, 1), createTile(colors.up, 2, 2, 1),
      createTile(colors.up, 0, 3, 1), createTile(colors.up, 1, 3, 1), createTile(colors.up, 2, 3, 1)
    ];
  
    // Down side (черный)
    const downSide = [
      createTile(colors.down, 0, -1, -1), createTile(colors.down, 1, -1, -1), createTile(colors.down, 2, -1, -1),
      createTile(colors.down, 0, -2, -1), createTile(colors.down, 1, -2, -1), createTile(colors.down, 2, -2, -1),
      createTile(colors.down, 0, -3, -1), createTile(colors.down, 1, -3, -1), createTile(colors.down, 2, -3, -1)
    ];
  
    // Добавляем все стороны в группу
    [frontSide, backSide, leftSide, rightSide, upSide, downSide].forEach(side => {
      side.forEach(tile => {
        cube.add(tile);
      });
    });
  
    scene.add(cube);
    camera.position.z = 10;
  
    // Рендеринг
    const animate = () => {
      requestAnimationFrame(animate);
      renderer.render(scene, camera);
    };
    animate();
  
    cubeRef.current = cube;
  }, [colors.front, colors.back, colors.left, colors.right, colors.up, colors.down]); // Добавили все цвета как зависимости
  
  const rotateCube = (axis) => {
    setRotation((prev) => {
      let newRotation = { ...prev };
      if (axis === 'x') {
        newRotation.x += Math.PI / 2; // Поворот на 90 градусов по оси X
      }
      if (axis === 'y') {
        newRotation.y += Math.PI / 2; // Поворот на 90 градусов по оси Y
      }
      return newRotation;
    });
  };

  useEffect(() => {
    if (cubeRef.current) {
      cubeRef.current.rotation.set(rotation.x, rotation.y, rotation.z);
    }
  }, [rotation]);

  return (
    <div>
      <div ref={containerRef} style={{ width: '400px', height: '400px' }} />
      <div>
        <button onClick={() => rotateCube('x')}>Rotate X</button>
        <button onClick={() => rotateCube('y')}>Rotate Y</button>
      </div>
    </div>
  );
};

export default RubikCube;
