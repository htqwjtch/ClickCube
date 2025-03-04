use crate::modules::{adapter::Adapter, cube::Cube};

pub struct CubeClient {}

impl CubeClient {
    pub fn solve() -> Vec<[[String; 3]; 3]> {
        let colors = Adapter::get_adapted_colors();
        let mut cube = Cube::new(colors.clone());
        cube.solve();

        vec![
            cube.get_front().get_color(),
            cube.get_back().get_color(),
            cube.get_up().get_color(),
            cube.get_down().get_color(),
            cube.get_left().get_color(),
            cube.get_right().get_color(),
        ]
    }
}
