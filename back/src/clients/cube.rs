use crate::modules::{
    color_adapter::ColorAdapter, cube::Cube, cube_face::CubeFace, opticourier::OptiCourier,
};

pub struct CubeClient {}

impl CubeClient {
    pub fn solve() -> Vec<String> {
        let colors = ColorAdapter::transmit_adapted_colors();
        let cube_faces = colors
            .iter()
            .map(|color| CubeFace::new(CubeClient::convert(color.clone())))
            .collect();
        let mut cube = Cube::new(cube_faces);
        cube.solve();

        OptiCourier::transmit_optimized_instructions()
    }

    fn convert(vec: Vec<String>) -> [[String; 3]; 3] {
        vec.chunks_exact(3)
            .map(|chunk| [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
            .collect::<Vec<_>>()
            .try_into()
            .expect("Each inner Vec must have exactly 9 elements")
    }
}
