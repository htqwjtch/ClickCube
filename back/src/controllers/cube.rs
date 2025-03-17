use crate::clients::cube::CubeClient;

pub struct CubeController {}

impl CubeController {
    pub fn solve() -> Vec<String> {
        CubeClient::solve()
    }
}
