use crate::modules::{color_adapter::ColorAdapter, cube_face::CubeFace, handmade::HandMade};

pub struct HandMadeClient {}

impl HandMadeClient {
    pub fn turn_cube_face(colors: Vec<Vec<String>>, command: String) -> Vec<Vec<String>> {
        ColorAdapter::receive_raw_colors(colors.clone());
        let adapted_colors = ColorAdapter::transmit_adapted_colors();
        let cube_faces = adapted_colors
            .iter()
            .map(|face_colors| HandMadeClient::vec_string_to_cube_face(face_colors.clone()))
            .collect();
        HandMade::turn_cube_face(cube_faces, command)
            .iter()
            .map(|cube_face| HandMadeClient::cube_face_to_vec_string(cube_face.clone()))
            .collect()
    }

    fn vec_string_to_cube_face(vec: Vec<String>) -> CubeFace {
        CubeFace::new(
            vec.chunks_exact(3)
                .map(|chunk| [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
                .collect::<Vec<_>>()
                .try_into()
                .expect("Each inner Vec must have exactly 9 elements"),
        )
    }

    fn cube_face_to_vec_string(cube_face: CubeFace) -> Vec<String> {
        cube_face.get_color().iter().flatten().cloned().collect()
    }
}
