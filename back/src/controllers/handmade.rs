use crate::clients::handmade::HandMadeClient;

pub struct HandMadeController {}

impl HandMadeController {
    pub fn turn_cube_face(colors: Vec<Vec<String>>, command: String) -> Vec<Vec<String>> {
        HandMadeClient::turn_cube_face(colors, command)
    }
}
