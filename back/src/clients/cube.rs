use crate::modules::{adapter::Adapter, cube::Cube};

pub struct CubeClient {}

impl CubeClient {
    pub fn solve() -> Vec<String> {
        let colors = Adapter::get_adapted_colors();
        let mut cube = Cube::new(CubeClient::convert(colors.clone()));
        cube.solve();

        vec![String::new()]
    }

    fn convert(vec: Vec<Vec<String>>) -> Vec<[[String; 3]; 3]> {
        vec.into_iter()
            .map(|flat_vec| {
                let arr: [[String; 3]; 3] = flat_vec
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Each inner Vec must have exactly 9 elements");
                arr
            })
            .collect()
    }
}
