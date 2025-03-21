use crate::modules::cube::Cube;
use crate::modules::cube_face::CubeFace;

pub struct HandMade;

impl HandMade {
    pub fn turn_cube_face(cube_faces: Vec<CubeFace>, command: String) -> Vec<CubeFace> {
        let mut cube = Cube::new(cube_faces);

        cube.execute_command(command);

        vec![
            cube.get_front(),
            cube.get_back(),
            cube.get_up(),
            cube.get_down(),
            cube.get_left(),
            cube.get_right(),
        ]
    }
}