use crate::clients::cube;

pub struct CubeController {}

impl CubeController {
    pub fn solve() -> Vec<[[String; 3]; 3]> {
        cube::CubeClient::solve()
    }
}
