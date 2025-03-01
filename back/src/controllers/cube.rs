use crate::clients::cube;

pub struct CubeController {}

impl CubeController {
    pub fn solve(colors: Vec<[[String; 3]; 3]>) -> Vec<[[String; 3]; 3]> {
        cube::CubeClient::solve(colors)
    }
}
