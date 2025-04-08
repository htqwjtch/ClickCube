use crate::modules::{cube_face::CubeFace, opticourier::OptiCourier};

pub struct Cube {
    front: CubeFace,
    back: CubeFace,
    up: CubeFace,
    down: CubeFace,
    left: CubeFace,
    right: CubeFace,
}

impl Cube {
    pub fn new(mut cube_faces: Vec<CubeFace>) -> Self {
        let orange;
        let red;
        let yellow;
        let white;
        let green;
        let blue;
        if cube_faces.is_empty() {
            orange = [
                ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
                ["OG".to_string(), "O".to_string(), "OB".to_string()],
                ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
            ];
            cube_faces.push(CubeFace::new(orange));
            red = [
                ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
                ["RB".to_string(), "R".to_string(), "RG".to_string()],
                ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
            ];
            cube_faces.push(CubeFace::new(red));
            yellow = [
                ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
                ["YG".to_string(), "Y".to_string(), "YB".to_string()],
                ["YOG".to_string(), "YO".to_string(), "YBO".to_string()],
            ];
            cube_faces.push(CubeFace::new(yellow));
            white = [
                ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
                ["WG".to_string(), "W".to_string(), "WB".to_string()],
                ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
            ];
            cube_faces.push(CubeFace::new(white));
            green = [
                ["GRY".to_string(), "GY".to_string(), "GYO".to_string()],
                ["GR".to_string(), "G".to_string(), "GO".to_string()],
                ["GWR".to_string(), "GW".to_string(), "GOW".to_string()],
            ];
            cube_faces.push(CubeFace::new(green));
            blue = [
                ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
                ["BO".to_string(), "B".to_string(), "BR".to_string()],
                ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
            ];
            cube_faces.push(CubeFace::new(blue));
        }

        Cube {
            front: cube_faces[0].clone(),
            back: cube_faces[1].clone(),
            up: cube_faces[2].clone(),
            down: cube_faces[3].clone(),
            left: cube_faces[4].clone(),
            right: cube_faces[5].clone(),
        }
    }

    pub fn set_front(&mut self, front: CubeFace) {
        if front.get_color() == self.back.get_color() {
            self.up = self.rotate_face_clockwise(self.rotate_face_clockwise(self.up.clone()));
            self.down = self.rotate_face_clockwise(self.rotate_face_clockwise(self.down.clone()));

            let tmp_left = self.left.clone();
            self.set_left(self.right.clone());
            self.set_right(tmp_left);
            self.set_back(self.front.clone());
        } else if front.get_color() == self.up.get_color() {
            self.set_left(self.rotate_face_clockwise(self.left.clone()));
            self.set_right(self.rotate_face_counterclockwise(self.right.clone()));

            self.set_up(self.rotate_face_clockwise(self.rotate_face_clockwise(self.back.clone())));
            self.set_back(
                self.rotate_face_clockwise(self.rotate_face_clockwise(self.down.clone())),
            );
            self.set_down(self.get_front());
        } else if front.get_color() == self.down.get_color() {
            self.set_left(self.rotate_face_counterclockwise(self.left.clone()));
            self.set_right(self.rotate_face_clockwise(self.right.clone()));

            self.set_down(
                self.rotate_face_clockwise(self.rotate_face_clockwise(self.back.clone())),
            );
            self.set_back(self.rotate_face_clockwise(self.rotate_face_clockwise(self.up.clone())));
            self.set_up(self.front.clone());
        } else if front.get_color() == self.left.get_color() {
            self.set_up(self.rotate_face_counterclockwise(self.up.clone()));
            self.set_down(self.rotate_face_clockwise(self.down.clone()));

            self.set_left(self.back.clone());
            self.set_back(self.right.clone());
            self.set_right(self.front.clone());
        } else if front.get_color() == self.right.get_color() {
            self.set_up(self.rotate_face_clockwise(self.up.clone()));
            self.set_down(self.rotate_face_counterclockwise(self.down.clone()));

            self.set_right(self.back.clone());
            self.set_back(self.left.clone());
            self.set_left(self.front.clone());
        }
        self.front = front;
    }

    pub fn get_front(&self) -> CubeFace {
        self.front.clone()
    }

    pub fn set_back(&mut self, back: CubeFace) {
        self.back = back;
    }

    pub fn get_back(&self) -> CubeFace {
        self.back.clone()
    }

    pub fn set_up(&mut self, up: CubeFace) {
        self.up = up;
    }

    pub fn get_up(&self) -> CubeFace {
        self.up.clone()
    }

    pub fn set_down(&mut self, down: CubeFace) {
        self.down = down;
    }

    pub fn get_down(&self) -> CubeFace {
        self.down.clone()
    }

    pub fn set_left(&mut self, left: CubeFace) {
        self.left = left;
    }

    pub fn get_left(&self) -> CubeFace {
        self.left.clone()
    }

    pub fn set_right(&mut self, right: CubeFace) {
        self.right = right;
    }

    pub fn get_right(&self) -> CubeFace {
        self.right.clone()
    }

    fn rotate_front_clockwise(&mut self) {
        let color_of_up_face = self.up.get_color();
        let color_of_left_face = self.left.get_color();
        let color_of_down_face = self.down.get_color();
        let color_of_right_face = self.right.get_color();

        let new_color_of_up_face = [
            color_of_up_face[0].clone(),
            color_of_up_face[1].clone(),
            [
                color_of_left_face[2][2].clone(),
                color_of_left_face[1][2].clone(),
                color_of_left_face[0][2].clone(),
            ],
        ];
        self.set_up(CubeFace::new(new_color_of_up_face));

        let new_color_of_left_face = [
            [
                color_of_left_face[0][0].clone(),
                color_of_left_face[0][1].clone(),
                color_of_down_face[0][0].clone(),
            ],
            [
                color_of_left_face[1][0].clone(),
                color_of_left_face[1][1].clone(),
                color_of_down_face[0][1].clone(),
            ],
            [
                color_of_left_face[2][0].clone(),
                color_of_left_face[2][1].clone(),
                color_of_down_face[0][2].clone(),
            ],
        ];
        self.set_left(CubeFace::new(new_color_of_left_face));

        let new_color_of_down_face = [
            [
                color_of_right_face[2][0].clone(),
                color_of_right_face[1][0].clone(),
                color_of_right_face[0][0].clone(),
            ],
            color_of_down_face[1].clone(),
            color_of_down_face[2].clone(),
        ];
        self.set_down(CubeFace::new(new_color_of_down_face));

        let new_color_of_right_face = [
            [
                color_of_up_face[2][0].clone(),
                color_of_right_face[0][1].clone(),
                color_of_right_face[0][2].clone(),
            ],
            [
                color_of_up_face[2][1].clone(),
                color_of_right_face[1][1].clone(),
                color_of_right_face[1][2].clone(),
            ],
            [
                color_of_up_face[2][2].clone(),
                color_of_right_face[2][1].clone(),
                color_of_right_face[2][2].clone(),
            ],
        ];
        self.set_right(CubeFace::new(new_color_of_right_face));

        self.set_front(self.rotate_face_clockwise(self.front.clone()));
    }

    fn rotate_face_clockwise(&self, cube_face: CubeFace) -> CubeFace {
        let mut new_color = cube_face.get_color().clone();
        let n = new_color.len();
        for i in 0..n {
            for j in 0..n {
                new_color[j][n - i - 1] = cube_face.get_color()[i][j].clone();
            }
        }
        CubeFace::new(new_color)
    }

    fn rotate_front_counterclockwise(&mut self) {
        for _ in 0..3 {
            self.rotate_front_clockwise();
        }
    }

    fn rotate_face_counterclockwise(&self, cube_face: CubeFace) -> CubeFace {
        let mut new_face = cube_face;
        for _ in 0..3 {
            new_face = self.rotate_face_clockwise(new_face.clone());
        }
        new_face
    }

    pub fn solve(&mut self) -> bool {
        //what if some steps are already ready

        self.make_daisy();
        self.make_cross_of_down_face();
        self.make_first_layer();
        self.make_second_layer();
        self.make_cross_of_up_face();
        self.make_right_cross_of_up_face();
        self.make_corners_of_up_face();
        self.make_third_layer();

        self.println_all_faces();

        self.check_all_faces()
    }

    fn check_all_faces(&self) -> bool {
        let front_color = self.get_front().get_color();
        for i in 0..front_color.len() {
            if front_color[0][0] != front_color[i][0] {
                return false;
            }
        }
        let back_color = self.get_back().get_color();
        for i in 0..back_color.len() {
            if back_color[0][0] != back_color[i][0] {
                return false;
            }
        }
        let up_color = self.get_up().get_color();
        for i in 0..up_color.len() {
            if up_color[0][0] != up_color[i][0] {
                return false;
            }
        }
        let down_color = self.get_down().get_color();
        for i in 0..down_color.len() {
            if down_color[0][0] != down_color[i][0] {
                return false;
            }
        }
        let left_color = self.get_left().get_color();
        for i in 0..left_color.len() {
            if left_color[0][0] != left_color[i][0] {
                return false;
            }
        }
        let right_color = self.get_right().get_color();
        for i in 0..right_color.len() {
            if right_color[0][0] != right_color[i][0] {
                return false;
            }
        }

        true
    }

    fn println_all_faces(&mut self) {
        println!("\nFRONT:");
        self.println_face(&(self.get_front()));
        println!("\nBACK:");
        self.println_face(&(self.get_back()));

        println!("\nUP:");
        self.println_face(&(self.get_up()));
        println!("\nDOWN:");
        self.println_face(&(self.get_down()));

        println!("\nLEFT:");
        self.println_face(&(self.get_left()));
        println!("\nRIGHT:");
        self.println_face(&(self.get_right()));
        println!("");
    }

    fn println_face(&mut self, cube_face: &CubeFace) {
        println!("{:?}", cube_face.get_color()[0]);
        println!("{:?}", cube_face.get_color()[1]);
        println!("{:?}", cube_face.get_color()[2]);
    }

    fn make_daisy(&mut self) {
        let raw_instruction = self.put_edges_of_down_face_up();
        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_edges_of_down_face_up(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..12 {
            let mut command_to_put_edge_up = self.check_edges_of_down_face();
            if !command_to_put_edge_up.is_empty() {
                instruction = instruction + command_to_put_edge_up.as_str() + "Y'";
                self.execute_command(command_to_put_edge_up + "Y'");
                continue;
            }

            command_to_put_edge_up = self.check_edges_of_first_layer();
            if !command_to_put_edge_up.is_empty() {
                instruction = instruction + command_to_put_edge_up.as_str() + "Y'";
                self.execute_command(command_to_put_edge_up + "Y'");
                continue;
            }

            command_to_put_edge_up = self.check_edges_of_second_layer();
            if !command_to_put_edge_up.is_empty() {
                instruction = instruction + command_to_put_edge_up.as_str() + "Y'";
                self.execute_command(command_to_put_edge_up + "Y'");
                continue;
            }

            command_to_put_edge_up = self.check_edges_of_third_layer();
            if !command_to_put_edge_up.is_empty() {
                instruction = instruction + command_to_put_edge_up.as_str() + "Y'";
                self.execute_command(command_to_put_edge_up + "Y'");
                continue;
            }

            break;
        }

        instruction
    }

    fn check_edges_of_down_face(&mut self) -> String {
        let mut command_to_put_edge_up = String::new();

        let color_of_up_face = self.get_up().get_color();
        let color_of_down_face = self.get_down().get_color();

        let center_color_of_down_face = &self.get_down().get_color()[1][1][0..1];

        if &color_of_down_face[0][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F2";
        } else if &color_of_down_face[1][0][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L2";
        } else if &color_of_down_face[2][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B2";
        } else if &color_of_down_face[1][2][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R2";
        }

        command_to_put_edge_up
    }

    fn check_edges_of_first_layer(&mut self) -> String {
        let mut command_to_put_edge_up = String::new();

        let color_of_front_face = self.get_front().get_color();
        let color_of_back_face = self.get_back().get_color();
        let color_of_up_face = self.get_up().get_color();
        let color_of_left_face = self.get_left().get_color();
        let color_of_right_face = self.get_right().get_color();

        let center_color_of_down_face = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front_face[2][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F";
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L'";
        } else if &color_of_left_face[2][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L";
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B'";
        } else if &color_of_back_face[2][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B";
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R'";
        } else if &color_of_right_face[2][1][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R";
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F'";
        }

        command_to_put_edge_up
    }

    fn check_edges_of_second_layer(&mut self) -> String {
        let mut command_to_put_edge_up = String::new();

        let color_of_front_face = self.get_front().get_color();
        let color_of_back_face = self.get_back().get_color();
        let color_of_up_face = self.get_up().get_color();
        let color_of_left_face = self.get_left().get_color();
        let color_of_right_face = self.get_right().get_color();

        let center_color_of_down_face = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front_face[1][0][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L'";
        } else if &color_of_front_face[1][2][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R";
        } else if &color_of_left_face[1][0][0..1] == center_color_of_down_face {
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B'";
        } else if &color_of_left_face[1][2][0..1] == center_color_of_down_face {
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F";
        } else if &color_of_back_face[1][0][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R'";
        } else if &color_of_back_face[1][2][0..1] == center_color_of_down_face {
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L";
        } else if &color_of_right_face[1][0][0..1] == center_color_of_down_face {
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F'";
        } else if &color_of_right_face[1][2][0..1] == center_color_of_down_face {
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B";
        }

        command_to_put_edge_up
    }

    fn check_edges_of_third_layer(&mut self) -> String {
        let mut command_to_put_edge_up = String::new();

        let color_of_front_face = self.get_front().get_color();
        let color_of_back_face = self.get_back().get_color();
        let color_of_up_face = self.get_up().get_color();
        let color_of_left_face = self.get_left().get_color();
        let color_of_right_face = self.get_right().get_color();

        let center_color_of_down_face = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front_face[0][1][0..1] == center_color_of_down_face {
            command_to_put_edge_up += "F'";
            if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "L'";
        } else if &color_of_left_face[0][1][0..1] == center_color_of_down_face {
            command_to_put_edge_up += "L'";
            if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "B'";
        } else if &color_of_back_face[0][1][0..1] == center_color_of_down_face {
            command_to_put_edge_up += "B'";
            if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[1][0][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "R'";
        } else if &color_of_right_face[0][1][0..1] == center_color_of_down_face {
            command_to_put_edge_up += "R'";
            if &color_of_up_face[2][1][0..1] == center_color_of_down_face {
                command_to_put_edge_up += "U";
                let mut times = 1;
                if &color_of_up_face[1][2][0..1] == center_color_of_down_face {
                    times += 1;
                    if &color_of_up_face[0][1][0..1] == center_color_of_down_face {
                        times = 1;
                        command_to_put_edge_up += "'";
                    }
                }
                if times > 1 {
                    command_to_put_edge_up += times.to_string().as_str();
                }
            }
            command_to_put_edge_up += "F'";
        }

        command_to_put_edge_up
    }

    pub fn execute_command(&mut self, command: String) {
        let mut i = 0;
        while i < command.len() {
            if &command[i..i + 1] == "F" {
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
            } else if &command[i..i + 1] == "B" {
                self.set_front(self.get_back());
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
                self.set_front(self.get_back());
            } else if &command[i..i + 1] == "U" {
                self.set_front(self.get_up());
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
                self.set_front(self.get_down());
            } else if &command[i..i + 1] == "D" {
                self.set_front(self.get_down());
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
                self.set_front(self.get_up());
            } else if &command[i..i + 1] == "L" {
                self.set_front(self.get_left());
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
                self.set_front(self.get_right());
            } else if &command[i..i + 1] == "R" {
                self.set_front(self.get_right());
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.rotate_front_counterclockwise();
                    } else if &command[i + 1..i + 2] == "2" {
                        self.rotate_front_clockwise();
                        self.rotate_front_clockwise();
                    } else {
                        self.rotate_front_clockwise();
                    }
                } else {
                    self.rotate_front_clockwise();
                }
                self.set_front(self.get_left());
            } else if &command[i..i + 1] == "X" {
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.set_front(self.get_up());
                    } else if &command[i + 1..i + 2] == "2" {
                        self.set_front(self.get_down());
                        self.set_front(self.get_down());
                    } else {
                        self.set_front(self.get_down());
                    }
                } else {
                    self.set_front(self.get_down());
                }
            } else if &command[i..i + 1] == "Y" {
                if i < command.len() - 1 {
                    if &command[i + 1..i + 2] == "'" {
                        self.set_front(self.get_left());
                    } else if &command[i + 1..i + 2] == "2" {
                        self.set_front(self.get_back());
                    } else {
                        self.set_front(self.get_right());
                    }
                } else {
                    self.set_front(self.get_right());
                }
            } else if &command[i..i + 1] == "(" {
                i += 1;
                let mut subcommand = String::new();
                while &command[i..i + 2] != ")x" {
                    subcommand += &command[i..i + 1];
                    i += 1;
                }
                i += 2;
                let mut times: i32 = command[i..i + 1]
                    .to_string()
                    .parse()
                    .expect("Failed to parse String to i32");
                while times > 0 {
                    self.execute_command(subcommand.clone());
                    times -= 1;
                }
            }
            i += 1;
        }
    }

    fn make_cross_of_down_face(&mut self) {
        let raw_instruction = self.put_edges_of_up_face_down();
        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_edges_of_up_face_down(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..4 {
            for _ in 0..4 {
                //TO CHECK MOMENT WHEN ARE SOME FACES ALREADY READY
                if self.get_up().get_color()[2][1][0..1] == self.get_down().get_color()[1][1][0..1]
                    && self.get_front().get_color()[0][1][0..1]
                        == self.get_front().get_color()[1][1][0..1]
                {
                    instruction += "F2";
                    self.execute_command("F2".to_string());
                }
                instruction += "Y'";
                self.execute_command("Y'".to_string());
            }
            let command_to_put_lower_edge_of_up_face_down =
                self.check_same_center_and_upper_edge_of_front_face();
            instruction = instruction + command_to_put_lower_edge_of_up_face_down.as_str() + "Y'";
            self.execute_command(command_to_put_lower_edge_of_up_face_down + "Y'");
        }
        instruction
    }

    fn check_same_center_and_upper_edge_of_front_face(&mut self) -> String {
        let mut command_to_put_edge_down = String::new();

        let color_of_back_face = self.get_back().get_color();
        let color_of_left_face = self.get_left().get_color();
        let color_of_right_face = self.get_right().get_color();

        let center_color_of_front_face = &self.get_front().get_color()[1][1][0..1];
        let center_color_of_down_face = &self.get_down().get_color()[1][1][0..1];

        if &self.get_up().get_color()[1][2][0..1] == center_color_of_down_face
            && &color_of_right_face[0][1][0..1] == center_color_of_front_face
        {
            command_to_put_edge_down += "UF2";
        } else if &self.get_up().get_color()[1][0][0..1] == center_color_of_down_face
            && &color_of_left_face[0][1][0..1] == center_color_of_front_face
        {
            command_to_put_edge_down += "U'F2";
        } else if &self.get_up().get_color()[0][1][0..1] == center_color_of_down_face
            && &color_of_back_face[0][1][0..1] == center_color_of_front_face
        {
            command_to_put_edge_down += "U2F2";
        }
        command_to_put_edge_down
    }

    fn make_first_layer(&mut self) {
        let instruction_1 = self.put_corners_of_down_face_up();
        let instruction_2 = self.put_corners_of_up_face_down();

        let raw_instruction = instruction_1 + instruction_2.as_str();
        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_corners_of_down_face_up(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..2 {
            for _ in 0..2 {
                let command_to_put_corner_up = self.check_corners_of_down_face();
                if !command_to_put_corner_up.is_empty() {
                    instruction += command_to_put_corner_up.as_str();
                    self.execute_command(command_to_put_corner_up);
                }
            }
            instruction += "Y2";
            self.execute_command("Y2".to_string());
        }
        instruction
    }

    fn check_corners_of_down_face(&mut self) -> String {
        let mut command_to_put_corner_up = String::new();

        let color_of_upper_left_corner_of_down_face = self.get_down().get_color()[0][0].clone();
        let color_of_upper_right_corner_of_down_face = self.get_down().get_color()[0][2].clone();

        let color_of_upper_left_corner_of_up_face = self.get_up().get_color()[0][0].clone();
        let color_of_upper_right_corner_of_up_face = self.get_up().get_color()[0][2].clone();
        let color_of_lower_left_corner_of_up_face = self.get_up().get_color()[2][0].clone();
        let color_of_lower_right_corner_of_up_face = self.get_up().get_color()[2][2].clone();

        let center_color_of_front_face = &self.get_front().get_color()[1][1][0..1];
        let center_color_of_left_face = &self.get_left().get_color()[1][1][0..1];
        let center_color_of_right_face = &self.get_right().get_color()[1][1][0..1];

        if color_of_upper_left_corner_of_down_face.contains("W")
            && ((&color_of_upper_left_corner_of_down_face[0..1] != "W")
                || (&color_of_upper_left_corner_of_down_face[0..1] == "W"
                    && (!color_of_upper_left_corner_of_down_face
                        .contains(center_color_of_left_face)
                        || !color_of_upper_left_corner_of_down_face
                            .contains(center_color_of_front_face))))
        {
            if color_of_lower_left_corner_of_up_face.contains("W") {
                if !color_of_lower_right_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U";
                } else if !color_of_upper_left_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U'";
                } else if !color_of_upper_right_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U2";
                }
            }
            command_to_put_corner_up += "L'U'LU";
        } else if color_of_upper_right_corner_of_down_face.contains("W")
            && (&color_of_upper_right_corner_of_down_face[0..1] != "W"
                || (&color_of_upper_right_corner_of_down_face[0..1] == "W"
                    && (!color_of_upper_right_corner_of_down_face
                        .contains(center_color_of_front_face)
                        || !color_of_upper_right_corner_of_down_face
                            .contains(center_color_of_right_face))))
        {
            if color_of_lower_right_corner_of_up_face.contains("W") {
                if !color_of_upper_right_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U";
                } else if !color_of_lower_left_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U'";
                } else if !color_of_upper_left_corner_of_up_face.contains("W") {
                    command_to_put_corner_up += "U2";
                }
            }
            command_to_put_corner_up += "RUR'U'";
        }

        command_to_put_corner_up
    }

    fn put_corners_of_up_face_down(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..2 {
            for _ in 0..2 {
                let command_to_put_corner_down = self.check_corners_of_up_face_1();
                if !command_to_put_corner_down.is_empty() {
                    instruction += command_to_put_corner_down.as_str();
                    self.execute_command(command_to_put_corner_down);
                }
            }
            instruction += "Y2";
            self.execute_command("Y2".to_string());
        }
        instruction
    }

    fn check_corners_of_up_face_1(&mut self) -> String {
        let mut command_to_put_corner_down = String::new();

        let upper_left_corner_of_up_face = self.get_up().get_color()[0][0].clone();
        let upper_right_corner_of_up_face = self.get_up().get_color()[0][2].clone();
        let lower_left_corner_of_up_face = self.get_up().get_color()[2][0].clone();
        let lower_right_corner_of_up_face = self.get_up().get_color()[2][2].clone();

        let color_of_center_of_front_face = &self.get_front().get_color()[1][1][0..1];
        let color_of_center_of_down_face = &self.get_down().get_color()[1][1][0..1];
        let color_of_center_of_left_face = &self.get_left().get_color()[1][1][0..1];
        let color_of_center_of_right_face = &self.get_right().get_color()[1][1][0..1];

        if lower_left_corner_of_up_face.contains(color_of_center_of_left_face)
            && lower_left_corner_of_up_face.contains(color_of_center_of_front_face)
            && lower_left_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &lower_left_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "(L'U'LU)x3";
            } else if &lower_left_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "Y'RUR'U'Y";
            } else if &lower_left_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "L'U'LU";
            }
        } else if lower_right_corner_of_up_face.contains(color_of_center_of_front_face)
            && lower_right_corner_of_up_face.contains(color_of_center_of_right_face)
            && lower_right_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &lower_right_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "(RUR'U')x3";
            } else if &lower_right_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "RUR'U'";
            } else if &lower_right_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "YL'U'LUY'";
            }
        } else if upper_left_corner_of_up_face.contains(color_of_center_of_left_face)
            && upper_left_corner_of_up_face.contains(color_of_center_of_front_face)
            && upper_left_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &upper_left_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'(L'U'LU)x3";
            } else if &upper_left_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'Y'RUR'U'Y";
            } else if &upper_left_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'L'U'LU";
            }
        } else if lower_right_corner_of_up_face.contains(color_of_center_of_front_face)
            && lower_right_corner_of_up_face.contains(color_of_center_of_left_face)
            && lower_right_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &lower_right_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U(L'U'LU)x3";
            } else if &lower_right_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "UY'RUR'U'Y";
            } else if &lower_right_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "UL'U'LU";
            }
        } else if lower_left_corner_of_up_face.contains(color_of_center_of_front_face)
            && lower_left_corner_of_up_face.contains(color_of_center_of_right_face)
            && lower_left_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &lower_left_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'(RUR'U')x3";
            } else if &lower_left_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'RUR'U'";
            } else if &lower_left_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "U'YL'U'LUY'";
            }
        } else if upper_right_corner_of_up_face.contains(color_of_center_of_right_face)
            && upper_right_corner_of_up_face.contains(color_of_center_of_front_face)
            && upper_right_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &upper_right_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U(RUR'U')x3";
            } else if &upper_right_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "URUR'U'";
            } else if &upper_right_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "UYL'U'LUY'";
            }
        } else if upper_right_corner_of_up_face.contains(color_of_center_of_left_face)
            && upper_right_corner_of_up_face.contains(color_of_center_of_front_face)
            && upper_right_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &upper_right_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2(L'U'LU)x3";
            } else if &upper_right_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2Y'RUR'U'Y";
            } else if &upper_right_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2L'U'LU";
            }
        } else if upper_left_corner_of_up_face.contains(color_of_center_of_right_face)
            && upper_left_corner_of_up_face.contains(color_of_center_of_front_face)
            && upper_left_corner_of_up_face.contains(color_of_center_of_down_face)
        {
            if &upper_left_corner_of_up_face[0..1] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2(RUR'U')x3";
            } else if &upper_left_corner_of_up_face[1..2] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2RUR'U'";
            } else if &upper_left_corner_of_up_face[2..3] == color_of_center_of_down_face {
                command_to_put_corner_down += "U2YL'U'LUY'";
            }
        }

        command_to_put_corner_down
    }

    fn make_second_layer(&mut self) {
        let instruction_1 = self.put_edges_of_second_layer_up();
        let instruction_2 = self.put_edges_of_second_layer_in_place();

        let raw_instruction = instruction_1 + instruction_2.as_str();
        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_edges_of_second_layer_up(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..2 {
            for _ in 0..2 {
                let command_to_put_edge_up = self.check_edges_of_second_layer_of_front_face();
                if !command_to_put_edge_up.is_empty() {
                    instruction += command_to_put_edge_up.as_str();
                    self.execute_command(command_to_put_edge_up);
                }
            }
            instruction += "Y2";
            self.execute_command("Y2".to_string());
        }
        instruction
    }

    fn check_edges_of_second_layer_of_front_face(&mut self) -> String {
        let mut command_to_put_edge_up = String::new();

        let color_of_center_of_front_face = self.get_front().get_color()[1][1].clone();
        let color_of_center_of_up_face = self.get_up().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        let left_edge_of_front_face = self.get_front().get_color()[1][0].clone();
        let right_edge_of_front_face = self.get_front().get_color()[1][2].clone();

        let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
        let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
        let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
        let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

        if !left_edge_of_front_face.contains(color_of_center_of_up_face.as_str())
            && !(left_edge_of_front_face[0..1] == color_of_center_of_front_face
                && left_edge_of_front_face[1..2] == color_of_center_of_left_face)
        {
            if upper_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "Y'RUR'U'YL'U'LU";
            } else if left_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "UY'RUR'U'YL'U'LU";
            } else if right_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "U'Y'RUR'U'YL'U'LU";
            } else if lower_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "U2Y'RUR'U'YL'U'LU";
            }
        } else if !right_edge_of_front_face.contains(color_of_center_of_up_face.as_str())
            && !(right_edge_of_front_face[0..1] == color_of_center_of_front_face
                && right_edge_of_front_face[1..2] == color_of_center_of_right_face)
        {
            if left_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "RUR'U'YL'U'LUY'";
            } else if lower_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "URUR'U'YL'U'LUY'";
            } else if upper_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "U'RUR'U'YL'U'LUY'";
            } else if right_edge_of_up_face.contains(color_of_center_of_up_face.as_str()) {
                command_to_put_edge_up += "U2RUR'U'YL'U'LUY'";
            }
        }

        command_to_put_edge_up
    }

    fn put_edges_of_second_layer_in_place(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..2 {
            for _ in 0..2 {
                let command_to_put_edge_in_place = self.check_edges_of_up_face_1();
                if !command_to_put_edge_in_place.is_empty() {
                    instruction += command_to_put_edge_in_place.as_str();
                    self.execute_command(command_to_put_edge_in_place);
                }
            }
            instruction += "Y2";
            self.execute_command("Y2".to_string());
        }
        instruction
    }

    fn check_edges_of_up_face_1(&mut self) -> String {
        let mut command_to_put_edge_in_place = String::new();

        let color_of_center_of_front_face = self.get_front().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
        let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
        let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
        let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

        if upper_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && upper_edge_of_up_face.contains(color_of_center_of_left_face.as_str())
        {
            if &upper_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &upper_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str()
            {
                command_to_put_edge_in_place += "Y'RUR'U'YL'U'LU";
            } else {
                command_to_put_edge_in_place += "UL'U'LUY'RUR'U'Y";
            }
        } else if upper_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && upper_edge_of_up_face.contains(color_of_center_of_right_face.as_str())
        {
            if &upper_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &upper_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str()
            {
                command_to_put_edge_in_place += "YL'U'LUY'RUR'U'";
            } else {
                command_to_put_edge_in_place += "U'RUR'U'YL'U'LUY'";
            }
        } else if left_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && left_edge_of_up_face.contains(color_of_center_of_left_face.as_str())
        {
            if &left_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &left_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str()
            {
                command_to_put_edge_in_place += "UY'RUR'U'YL'U'LU";
            } else {
                command_to_put_edge_in_place += "U2L'U'LUY'RUR'U'Y";
            }
        } else if left_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && left_edge_of_up_face.contains(color_of_center_of_right_face.as_str())
        {
            if &left_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &left_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str()
            {
                command_to_put_edge_in_place += "UYL'U'LUY'RUR'U'";
            } else {
                command_to_put_edge_in_place += "RUR'U'YL'U'LUY'";
            }
        } else if lower_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && lower_edge_of_up_face.contains(color_of_center_of_left_face.as_str())
        {
            if &lower_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &lower_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str()
            {
                command_to_put_edge_in_place += "U2Y'RUR'U'YL'U'LU";
            } else {
                command_to_put_edge_in_place += "U'L'U'LUY'RUR'U'Y";
            }
        } else if lower_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && lower_edge_of_up_face.contains(color_of_center_of_right_face.as_str())
        {
            if &lower_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &lower_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str()
            {
                command_to_put_edge_in_place += "U2YL'U'LUY'RUR'U'";
            } else {
                command_to_put_edge_in_place += "URUR'U'YL'U'LUY'";
            }
        } else if right_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && right_edge_of_up_face.contains(color_of_center_of_left_face.as_str())
        {
            if &right_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &right_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str()
            {
                command_to_put_edge_in_place += "U'Y'RUR'U'YL'U'LU";
            } else {
                command_to_put_edge_in_place += "L'U'LUY'RUR'U'Y";
            }
        } else if right_edge_of_up_face.contains(color_of_center_of_front_face.as_str())
            && right_edge_of_up_face.contains(color_of_center_of_right_face.as_str())
        {
            if &right_edge_of_up_face[0..1] == color_of_center_of_front_face.as_str()
                && &right_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str()
            {
                command_to_put_edge_in_place += "U'YL'U'LUY'RUR'U'";
            } else {
                command_to_put_edge_in_place += "U2RUR'U'YL'U'LUY'";
            }
        }

        command_to_put_edge_in_place
    }

    fn make_cross_of_up_face(&mut self) {
        let raw_instruction = self.put_edges_of_up_face_in_cross();

        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_edges_of_up_face_in_cross(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..4 {
            let command_to_make_cross = self.check_edges_of_up_face_2();
            if !command_to_make_cross.is_empty() {
                let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
                let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
                let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
                let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

                let color_of_center_of_up_face = self.get_up().get_color()[1][1].clone();

                if &upper_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
                    && &left_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
                    && &lower_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
                    && &right_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
                {
                    instruction += command_to_make_cross.as_str();
                    self.execute_command(command_to_make_cross);
                    break;
                }
                instruction += command_to_make_cross.as_str();
                self.execute_command(command_to_make_cross);
            }
        }
        instruction
    }

    fn check_edges_of_up_face_2(&mut self) -> String {
        let mut command_to_make_cross = String::new();

        let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
        let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
        let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
        let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

        let color_of_center_of_up_face = self.get_up().get_color()[1][1].clone();

        if &upper_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
            if &left_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
                if &lower_edge_of_up_face[0..1] != color_of_center_of_up_face.as_str()
                    && &right_edge_of_up_face[0..1] != color_of_center_of_up_face.as_str()
                {
                    command_to_make_cross += "FRUR'U'F'";
                }
            } else if &lower_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
                || &right_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str()
            {
                command_to_make_cross += "U'FRUR'U'F'";
            }
        } else if &left_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
            if &right_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
                command_to_make_cross += "FRUR'U'F'";
            } else if &lower_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
                command_to_make_cross += "UFRUR'U'F'";
            }
        } else if &lower_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
            if &right_edge_of_up_face[0..1] == color_of_center_of_up_face.as_str() {
                command_to_make_cross += "U2FRUR'U'F'";
            }
        } else {
            command_to_make_cross += "FRUR'U'F'";
        }

        command_to_make_cross
    }

    fn make_right_cross_of_up_face(&mut self) {
        let raw_instruction = self.put_edges_of_up_face_in_place();

        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_edges_of_up_face_in_place(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..12 {
            let command_to_put_edges_in_place = self.check_edges_of_up_face_3();
            if !command_to_put_edges_in_place.is_empty() {
                instruction += command_to_put_edges_in_place.as_str();
                self.execute_command(command_to_put_edges_in_place);
            }
            if self.check_edges_of_up_face_4() {
                break;
            }
            instruction += "U";
            self.execute_command("U".to_string());
        }
        instruction
    }

    fn check_edges_of_up_face_3(&mut self) -> String {
        let mut command_to_make_cross = String::new();

        let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
        let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
        let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
        let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

        let color_of_center_of_front_face = self.get_front().get_color()[1][1].clone();
        let color_of_center_of_back_face = self.get_back().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        if &upper_edge_of_up_face[1..2] == color_of_center_of_back_face.as_str() {
            if &right_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str() {
                if &lower_edge_of_up_face[1..2] != color_of_center_of_front_face.as_str()
                    && &left_edge_of_up_face[1..2] != color_of_center_of_left_face.as_str()
                {
                    command_to_make_cross += "RUR'URU2R'U";
                }
            } else if &left_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str() {
                command_to_make_cross += "URUR'URU2R'U";
            } else if &lower_edge_of_up_face[1..2] == color_of_center_of_front_face.as_str() {
                command_to_make_cross += "RUR'URU2R'";
            }
        } else if &left_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str() {
            if &right_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str() {
                command_to_make_cross += "URUR'URU2R'";
            } else if &lower_edge_of_up_face[1..2] == color_of_center_of_front_face.as_str() {
                command_to_make_cross += "U2RUR'URU2R'U";
            }
        } else if &lower_edge_of_up_face[1..2] == color_of_center_of_front_face.as_str() {
            if &right_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str() {
                command_to_make_cross += "U'RUR'URU2R'U";
            }
        }

        command_to_make_cross
    }

    fn check_edges_of_up_face_4(&mut self) -> bool {
        let upper_edge_of_up_face = self.get_up().get_color()[0][1].clone();
        let lower_edge_of_up_face = self.get_up().get_color()[2][1].clone();
        let left_edge_of_up_face = self.get_up().get_color()[1][0].clone();
        let right_edge_of_up_face = self.get_up().get_color()[1][2].clone();

        let color_of_center_of_front_face = self.get_front().get_color()[1][1].clone();
        let color_of_center_of_back_face = self.get_back().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        &upper_edge_of_up_face[1..2] == color_of_center_of_back_face.as_str()
            && &right_edge_of_up_face[1..2] == color_of_center_of_right_face.as_str()
            && &lower_edge_of_up_face[1..2] == color_of_center_of_front_face.as_str()
            && &left_edge_of_up_face[1..2] == color_of_center_of_left_face.as_str()
    }

    fn make_corners_of_up_face(&mut self) {
        let raw_instruction = self.put_corners_of_up_face_in_place();
        OptiCourier::receive_raw_instruction(raw_instruction);
    }

    fn put_corners_of_up_face_in_place(&mut self) -> String {
        let mut instruction = String::new();
        for _ in 0..12 {
            let command_to_put_corners_in_place = self.check_corners_of_up_face_2();
            if !command_to_put_corners_in_place.is_empty() {
                instruction += command_to_put_corners_in_place.as_str();
                self.execute_command(command_to_put_corners_in_place);
            } else {
                break;
            }
        }
        let command_to_put_up_face_correctly = self.check_center_and_upper_edge_of_front_face();
        if !command_to_put_up_face_correctly.is_empty() {
            instruction += command_to_put_up_face_correctly.as_str();
            self.execute_command(command_to_put_up_face_correctly);
        }
        instruction
    }

    fn check_corners_of_up_face_2(&mut self) -> String {
        let mut command_to_put_corners_in_place = String::new();

        let upper_left_corner_of_up_face = self.get_up().get_color()[0][0].clone();
        let upper_right_corner_of_up_face = self.get_up().get_color()[0][2].clone();
        let lower_left_corner_of_up_face = self.get_up().get_color()[2][0].clone();
        let lower_right_corner_of_up_face = self.get_up().get_color()[2][2].clone();

        let second_color_of_upper_edge_of_up_face = &self.get_up().get_color()[0][1][1..2];
        let second_color_of_lower_edge_of_up_face = &self.get_up().get_color()[2][1][1..2];
        let second_color_of_left_edge_of_up_face = &self.get_up().get_color()[1][0][1..2];
        let second_color_of_right_edge_of_up_face = &self.get_up().get_color()[1][2][1..2];

        if upper_left_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face)
            && upper_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face)
        {
            if !(lower_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face)
                && lower_left_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face))
                && !(lower_right_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face)
                    && lower_right_corner_of_up_face
                        .contains(second_color_of_right_edge_of_up_face))
                && !(upper_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face)
                    && upper_right_corner_of_up_face
                        .contains(second_color_of_upper_edge_of_up_face))
            {
                command_to_put_corners_in_place += "U'RU'L'UR'U'LU";
            }
        } else if lower_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face)
            && lower_left_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face)
        {
            if !(lower_right_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face)
                && lower_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face))
                && !(upper_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face)
                    && upper_right_corner_of_up_face
                        .contains(second_color_of_upper_edge_of_up_face))
                && !(upper_left_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face)
                    && upper_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face))
            {
                command_to_put_corners_in_place += "RU'L'UR'U'LU";
            }
        } else if lower_right_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face)
            && lower_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face)
        {
            if !(upper_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face)
                && upper_right_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face))
                && !(upper_left_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face)
                    && upper_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face))
                && !(lower_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face)
                    && lower_left_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face))
            {
                command_to_put_corners_in_place += "URU'L'UR'U'LU";
            }
        } else if upper_right_corner_of_up_face.contains(second_color_of_right_edge_of_up_face)
            && upper_right_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face)
        {
            if !(upper_left_corner_of_up_face.contains(second_color_of_upper_edge_of_up_face)
                && upper_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face))
                || !(lower_left_corner_of_up_face.contains(second_color_of_left_edge_of_up_face)
                    && lower_left_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face))
                || !(lower_right_corner_of_up_face.contains(second_color_of_lower_edge_of_up_face)
                    && lower_right_corner_of_up_face
                        .contains(second_color_of_right_edge_of_up_face))
            {
                command_to_put_corners_in_place += "U2RU'L'UR'U'LU";
            }
        } else {
            command_to_put_corners_in_place += "RU'L'UR'U'LU";
        }

        command_to_put_corners_in_place
    }

    fn check_center_and_upper_edge_of_front_face(&mut self) -> String {
        let mut command_to_put_up_face_correctly = String::new();

        let upper_edge_of_front_face = self.get_front().get_color()[0][1].clone();

        let color_of_center_of_back_face = self.get_back().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        if upper_edge_of_front_face.contains(color_of_center_of_back_face.as_str()) {
            command_to_put_up_face_correctly += "U2";
        } else if upper_edge_of_front_face.contains(color_of_center_of_left_face.as_str()) {
            command_to_put_up_face_correctly += "U";
        } else if upper_edge_of_front_face.contains(color_of_center_of_right_face.as_str()) {
            command_to_put_up_face_correctly += "U'";
        }

        command_to_put_up_face_correctly
    }

    fn make_third_layer(&mut self) {
        let raw_instruction = self.put_corners_of_up_face_correctly();
        OptiCourier::receive_raw_instruction(raw_instruction.clone());
    }

    fn put_corners_of_up_face_correctly(&mut self) -> String {
        let mut instruction = String::from("X2");
        self.execute_command("X2".to_string());

        for _ in 0..4 {
            let command_to_put_corners_correctly = self.check_corners_of_up_face_3();
            if !command_to_put_corners_correctly.is_empty() {
                instruction += command_to_put_corners_correctly.as_str();
                self.execute_command(command_to_put_corners_correctly);
            }
        }
        let command_to_put_down_face_correctly = self.check_center_and_lower_edge_of_front_face();
        if !command_to_put_down_face_correctly.is_empty() {
            instruction += command_to_put_down_face_correctly.as_str();
            self.execute_command(command_to_put_down_face_correctly);
        }
        instruction
    }

    fn check_corners_of_up_face_3(&mut self) -> String {
        let mut command_to_put_corners_correctly = String::new();

        let upper_right_corner_of_down_face = self.get_down().get_color()[0][2].clone();

        let color_of_center_of_down_face = self.get_down().get_color()[1][1].clone();

        if &upper_right_corner_of_down_face[1..2] == color_of_center_of_down_face {
            command_to_put_corners_correctly += "(RUR'U')x4";
        } else if &upper_right_corner_of_down_face[2..3] == color_of_center_of_down_face {
            command_to_put_corners_correctly += "(RUR'U')x2";
        }
        command_to_put_corners_correctly += "D";

        command_to_put_corners_correctly
    }

    fn check_center_and_lower_edge_of_front_face(&mut self) -> String {
        let mut command_to_put_down_face_correctly = String::new();

        let lower_edge_of_front_face = self.get_front().get_color()[2][1].clone();

        let color_of_center_of_back_face = self.get_back().get_color()[1][1].clone();
        let color_of_center_of_left_face = self.get_left().get_color()[1][1].clone();
        let color_of_center_of_right_face = self.get_right().get_color()[1][1].clone();

        if lower_edge_of_front_face.contains(color_of_center_of_back_face.as_str()) {
            command_to_put_down_face_correctly += "D2";
        } else if lower_edge_of_front_face.contains(color_of_center_of_left_face.as_str()) {
            command_to_put_down_face_correctly += "D'";
        } else if lower_edge_of_front_face.contains(color_of_center_of_right_face.as_str()) {
            command_to_put_down_face_correctly += "D";
        }

        command_to_put_down_face_correctly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_front() {
        let mut cube = Cube::new(Vec::new());
        let actual_front = cube.get_front().get_color();
        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let expected_front = CubeFace::new(orange).get_color();
        assert_eq!(actual_front, expected_front);

        cube.set_front(cube.get_right());
        let actual_right = cube.get_front().get_color();
        let blue = [
            ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
            ["BO".to_string(), "B".to_string(), "BR".to_string()],
            ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        let expected_right = CubeFace::new(blue).get_color();
        assert_eq!(actual_right, expected_right);

        cube.set_front(cube.get_down());
        let actual_back = cube.get_front().get_color();
        let red = [
            ["WOB".to_string(), "WB".to_string(), "WBR".to_string()],
            ["WO".to_string(), "W".to_string(), "WR".to_string()],
            ["WGO".to_string(), "WG".to_string(), "WRG".to_string()],
        ];
        let expected_back = CubeFace::new(red).get_color();
        assert_eq!(actual_back, expected_back);

        cube.set_front(cube.get_back());
        let actual_left = cube.get_front().get_color();
        let green = [
            ["YRB".to_string(), "YB".to_string(), "YBO".to_string()],
            ["YR".to_string(), "Y".to_string(), "YO".to_string()],
            ["YGR".to_string(), "YG".to_string(), "YOG".to_string()],
        ];
        let expected_left = CubeFace::new(green).get_color();
        assert_eq!(actual_left, expected_left);

        cube.set_front(cube.get_back());
        let actual_back = cube.get_front().get_color();
        let red = [
            ["WOB".to_string(), "WB".to_string(), "WBR".to_string()],
            ["WO".to_string(), "W".to_string(), "WR".to_string()],
            ["WGO".to_string(), "WG".to_string(), "WRG".to_string()],
        ];
        let expected_back = CubeFace::new(red).get_color();
        assert_eq!(actual_back, expected_back);

        cube.set_front(cube.get_back());
        let actual_left = cube.get_front().get_color();
        let green = [
            ["YRB".to_string(), "YB".to_string(), "YBO".to_string()],
            ["YR".to_string(), "Y".to_string(), "YO".to_string()],
            ["YGR".to_string(), "YG".to_string(), "YOG".to_string()],
        ];
        let expected_left = CubeFace::new(green).get_color();
        assert_eq!(actual_left, expected_left);

        cube.set_front(cube.get_up());
        let actual_up = cube.get_front().get_color();
        let yellow = [
            ["BRW".to_string(), "BW".to_string(), "BWO".to_string()],
            ["BR".to_string(), "B".to_string(), "BO".to_string()],
            ["BYR".to_string(), "BY".to_string(), "BOY".to_string()],
        ];
        let expected_up = CubeFace::new(yellow).get_color();
        assert_eq!(actual_up, expected_up);

        cube.set_front(cube.get_left());
        let actual_down = cube.get_front().get_color();
        let white = [
            ["RGW".to_string(), "RW".to_string(), "RWB".to_string()],
            ["RG".to_string(), "R".to_string(), "RB".to_string()],
            ["RYG".to_string(), "RY".to_string(), "RBY".to_string()],
        ];
        let expected_down = CubeFace::new(white).get_color();
        assert_eq!(actual_down, expected_down);
    }

    #[test]
    fn test_clockwise_x1_x2_x3_x4() {
        let mut cube = Cube::new(Vec::new());
        let expected_front_x0 = cube.get_front().get_color();
        let expected_back_x0 = cube.get_back().get_color();
        let expected_up_x0 = cube.get_up().get_color();
        let expected_down_x0 = cube.get_down().get_color();
        let expected_left_x0 = cube.get_left().get_color();
        let expected_right_x0 = cube.get_right().get_color();

        cube.rotate_front_clockwise();

        let actual_front_x1 = cube.get_front().get_color();
        let expected_front_x1 = [
            ["OWG".to_string(), "OG".to_string(), "OGY".to_string()],
            ["OW".to_string(), "O".to_string(), "OY".to_string()],
            ["OBW".to_string(), "OB".to_string(), "OYB".to_string()],
        ];
        assert_eq!(actual_front_x1, expected_front_x1);

        let actual_back_x1 = cube.get_back().get_color();
        let expected_back_x1 = expected_back_x0;
        assert_eq!(actual_back_x1, expected_back_x1);

        let actual_up_x1 = cube.get_up().get_color();
        let expected_up_x1 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["GOW".to_string(), "GO".to_string(), "GYO".to_string()],
        ];
        assert_eq!(actual_up_x1, expected_up_x1);

        let actual_down_x1 = cube.get_down().get_color();
        let expected_down_x1 = [
            ["BWO".to_string(), "BO".to_string(), "BOY".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x1, expected_down_x1);

        let actual_left_x1 = cube.get_left().get_color();
        let expected_left_x1 = [
            ["GRY".to_string(), "GY".to_string(), "WGO".to_string()],
            ["GR".to_string(), "G".to_string(), "WO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_left_x1, expected_left_x1);

        let actual_right_x1 = cube.get_right().get_color();
        let expected_right_x1 = [
            ["YOG".to_string(), "BY".to_string(), "BYR".to_string()],
            ["YO".to_string(), "B".to_string(), "BR".to_string()],
            ["YBO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x1, expected_right_x1);

        cube.rotate_front_clockwise();

        let actual_front_x2 = cube.get_front().get_color();
        let expected_front_x2 = [
            ["OBW".to_string(), "OW".to_string(), "OWG".to_string()],
            ["OB".to_string(), "O".to_string(), "OG".to_string()],
            ["OYB".to_string(), "OY".to_string(), "OGY".to_string()],
        ];
        assert_eq!(actual_front_x2, expected_front_x2);

        let actual_back_x2 = cube.get_back().get_color();
        let expected_back_x2 = expected_back_x1;
        assert_eq!(actual_back_x2, expected_back_x2);

        let actual_up_x2 = cube.get_up().get_color();
        let expected_up_x2 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["WOB".to_string(), "WO".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_up_x2, expected_up_x2);

        let actual_down_x2 = cube.get_down().get_color();
        let expected_down_x2 = [
            ["YBO".to_string(), "YO".to_string(), "YOG".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x2, expected_down_x2);

        let actual_left_x2 = cube.get_left().get_color();
        let expected_left_x2 = [
            ["GRY".to_string(), "GY".to_string(), "BWO".to_string()],
            ["GR".to_string(), "G".to_string(), "BO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "BOY".to_string()],
        ];
        assert_eq!(actual_left_x2, expected_left_x2);

        let actual_right_x2 = cube.get_right().get_color();
        let expected_right_x2 = [
            ["GOW".to_string(), "BY".to_string(), "BYR".to_string()],
            ["GO".to_string(), "B".to_string(), "BR".to_string()],
            ["GYO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x2, expected_right_x2);

        let mut cube1 = Cube::new(Vec::new());
        cube1.rotate_front_counterclockwise();

        let expected_front1 = cube1.get_front().get_color();
        let expected_back1 = cube1.get_back().get_color();
        let expected_up1 = cube1.get_up().get_color();
        let expected_down1 = cube1.get_down().get_color();
        let expected_left1 = cube1.get_left().get_color();
        let expected_right1 = cube1.get_right().get_color();

        cube.rotate_front_clockwise();

        let actual_front_x3 = cube.get_front().get_color();
        assert_eq!(actual_front_x3, expected_front1);
        let expected_front_x3 = [
            ["OYB".to_string(), "OB".to_string(), "OBW".to_string()],
            ["OY".to_string(), "O".to_string(), "OW".to_string()],
            ["OGY".to_string(), "OG".to_string(), "OWG".to_string()],
        ];
        assert_eq!(actual_front_x3, expected_front_x3);

        let actual_back_x3 = cube.get_back().get_color();
        assert_eq!(actual_back_x3, expected_back1);
        let expected_back_x3 = expected_back_x2;
        assert_eq!(actual_back_x3, expected_back_x3);

        let actual_up_x3 = cube.get_up().get_color();
        assert_eq!(actual_up_x3, expected_up1);
        let expected_up_x3 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["BOY".to_string(), "BO".to_string(), "BWO".to_string()],
        ];
        assert_eq!(actual_up_x3, expected_up_x3);

        let actual_down_x3 = cube.get_down().get_color();
        assert_eq!(actual_down_x3, expected_down1);
        let expected_down_x3 = [
            ["GYO".to_string(), "GO".to_string(), "GOW".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x3, expected_down_x3);

        let actual_left_x3 = cube.get_left().get_color();
        assert_eq!(actual_left_x3, expected_left1);
        let expected_left_x3 = [
            ["GRY".to_string(), "GY".to_string(), "YBO".to_string()],
            ["GR".to_string(), "G".to_string(), "YO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "YOG".to_string()],
        ];
        assert_eq!(actual_left_x3, expected_left_x3);

        let actual_right_x3 = cube.get_right().get_color();
        assert_eq!(actual_right_x3, expected_right1);
        let expected_right_x3 = [
            ["WOB".to_string(), "BY".to_string(), "BYR".to_string()],
            ["WO".to_string(), "B".to_string(), "BR".to_string()],
            ["WGO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x3, expected_right_x3);

        cube.rotate_front_clockwise();

        let actual_front_x4 = cube.get_front().get_color();
        let expected_front_x4 = expected_front_x0;
        assert_eq!(actual_front_x4, expected_front_x4);

        let actual_back_x4 = cube.get_back().get_color();
        let expected_back_x4 = expected_back_x3;
        assert_eq!(actual_back_x4, expected_back_x4);

        let actual_up_x4 = cube.get_up().get_color();
        let expected_up_x4 = expected_up_x0;
        assert_eq!(actual_up_x4, expected_up_x4);

        let actual_down_x4 = cube.get_down().get_color();
        let expected_down_x4 = expected_down_x0;
        assert_eq!(actual_down_x4, expected_down_x4);

        let actual_left_x4 = cube.get_left().get_color();
        let expected_left_x4 = expected_left_x0;
        assert_eq!(actual_left_x4, expected_left_x4);

        let actual_right_x4 = cube.get_right().get_color();
        let expected_right_x4 = expected_right_x0;
        assert_eq!(actual_right_x4, expected_right_x4);
    }

    #[test]
    fn test_counterclockwise_x1_x2_x3_x4() {
        let mut cube = Cube::new(Vec::new());
        let expected_front_x0 = cube.get_front().get_color();
        let expected_back_x0 = cube.get_back().get_color();
        let expected_up_x0 = cube.get_up().get_color();
        let expected_down_x0 = cube.get_down().get_color();
        let expected_left_x0 = cube.get_left().get_color();
        let expected_right_x0 = cube.get_right().get_color();

        cube.rotate_front_counterclockwise();

        let actual_front_x1 = cube.get_front().get_color();
        let expected_front_x1 = [
            ["OYB".to_string(), "OB".to_string(), "OBW".to_string()],
            ["OY".to_string(), "O".to_string(), "OW".to_string()],
            ["OGY".to_string(), "OG".to_string(), "OWG".to_string()],
        ];
        assert_eq!(actual_front_x1, expected_front_x1);

        let actual_back_x1 = cube.get_back().get_color();
        let expected_back_x1 = expected_back_x0;
        assert_eq!(actual_back_x1, expected_back_x1);

        let actual_up_x1 = cube.get_up().get_color();
        let expected_up_x1 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["BOY".to_string(), "BO".to_string(), "BWO".to_string()],
        ];
        assert_eq!(actual_up_x1, expected_up_x1);

        let actual_down_x1 = cube.get_down().get_color();
        let expected_down_x1 = [
            ["GYO".to_string(), "GO".to_string(), "GOW".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x1, expected_down_x1);

        let actual_left_x1 = cube.get_left().get_color();
        let expected_left_x1 = [
            ["GRY".to_string(), "GY".to_string(), "YBO".to_string()],
            ["GR".to_string(), "G".to_string(), "YO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "YOG".to_string()],
        ];
        assert_eq!(actual_left_x1, expected_left_x1);

        let actual_right_x1 = cube.get_right().get_color();
        let expected_right_x1 = [
            ["WOB".to_string(), "BY".to_string(), "BYR".to_string()],
            ["WO".to_string(), "B".to_string(), "BR".to_string()],
            ["WGO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x1, expected_right_x1);

        cube.rotate_front_counterclockwise();

        let actual_front_x2 = cube.get_front().get_color();
        let expected_front_x2 = [
            ["OBW".to_string(), "OW".to_string(), "OWG".to_string()],
            ["OB".to_string(), "O".to_string(), "OG".to_string()],
            ["OYB".to_string(), "OY".to_string(), "OGY".to_string()],
        ];
        assert_eq!(actual_front_x2, expected_front_x2);

        let actual_back_x2 = cube.get_back().get_color();
        let expected_back_x2 = expected_back_x1;
        assert_eq!(actual_back_x2, expected_back_x2);

        let actual_up_x2 = cube.get_up().get_color();
        let expected_up_x2 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["WOB".to_string(), "WO".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_up_x2, expected_up_x2);

        let actual_down_x2 = cube.get_down().get_color();
        let expected_down_x2 = [
            ["YBO".to_string(), "YO".to_string(), "YOG".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x2, expected_down_x2);

        let actual_left_x2 = cube.get_left().get_color();
        let expected_left_x2 = [
            ["GRY".to_string(), "GY".to_string(), "BWO".to_string()],
            ["GR".to_string(), "G".to_string(), "BO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "BOY".to_string()],
        ];
        assert_eq!(actual_left_x2, expected_left_x2);

        let actual_right_x2 = cube.get_right().get_color();
        let expected_right_x2 = [
            ["GOW".to_string(), "BY".to_string(), "BYR".to_string()],
            ["GO".to_string(), "B".to_string(), "BR".to_string()],
            ["GYO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x2, expected_right_x2);

        let mut cube1 = Cube::new(Vec::new());
        cube1.rotate_front_clockwise();

        let expected_front1 = cube1.get_front().get_color();
        let expected_back1 = cube1.get_back().get_color();
        let expected_up1 = cube1.get_up().get_color();
        let expected_down1 = cube1.get_down().get_color();
        let expected_left1 = cube1.get_left().get_color();
        let expected_right1 = cube1.get_right().get_color();

        cube.rotate_front_counterclockwise();

        let actual_front_x3 = cube.get_front().get_color();
        assert_eq!(actual_front_x3, expected_front1);
        let expected_front_x3 = [
            ["OWG".to_string(), "OG".to_string(), "OGY".to_string()],
            ["OW".to_string(), "O".to_string(), "OY".to_string()],
            ["OBW".to_string(), "OB".to_string(), "OYB".to_string()],
        ];
        assert_eq!(actual_front_x3, expected_front_x3);

        let actual_back_x3 = cube.get_back().get_color();
        assert_eq!(actual_back_x3, expected_back1);
        let expected_back_x3 = expected_back_x2;
        assert_eq!(actual_back_x3, expected_back_x3);

        let actual_up_x3 = cube.get_up().get_color();
        assert_eq!(actual_up_x3, expected_up1);
        let expected_up_x3 = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["GOW".to_string(), "GO".to_string(), "GYO".to_string()],
        ];
        assert_eq!(actual_up_x3, expected_up_x3);

        let actual_down_x3 = cube.get_down().get_color();
        assert_eq!(actual_down_x3, expected_down1);
        let expected_down_x3 = [
            ["BWO".to_string(), "BO".to_string(), "BOY".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_down_x3, expected_down_x3);

        let actual_left_x3 = cube.get_left().get_color();
        assert_eq!(actual_left_x3, expected_left1);
        let expected_left_x3 = [
            ["GRY".to_string(), "GY".to_string(), "WGO".to_string()],
            ["GR".to_string(), "G".to_string(), "WO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_left_x3, expected_left_x3);

        let actual_right_x3 = cube.get_right().get_color();
        assert_eq!(actual_right_x3, expected_right1);
        let expected_right_x3 = [
            ["YOG".to_string(), "BY".to_string(), "BYR".to_string()],
            ["YO".to_string(), "B".to_string(), "BR".to_string()],
            ["YBO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_right_x3, expected_right_x3);

        cube.rotate_front_counterclockwise();

        let actual_front_x4 = cube.get_front().get_color();
        let expected_front_x4 = expected_front_x0;
        assert_eq!(actual_front_x4, expected_front_x4);

        let actual_back_x4 = cube.get_back().get_color();
        let expected_back_x4 = expected_back_x3;
        assert_eq!(actual_back_x4, expected_back_x4);

        let actual_up_x4 = cube.get_up().get_color();
        let expected_up_x4 = expected_up_x0;
        assert_eq!(actual_up_x4, expected_up_x4);

        let actual_down_x4 = cube.get_down().get_color();
        let expected_down_x4 = expected_down_x0;
        assert_eq!(actual_down_x4, expected_down_x4);

        let actual_left_x4 = cube.get_left().get_color();
        let expected_left_x4 = expected_left_x0;
        assert_eq!(actual_left_x4, expected_left_x4);

        let actual_right_x4 = cube.get_right().get_color();
        let expected_right_x4 = expected_right_x0;
        assert_eq!(actual_right_x4, expected_right_x4);
    }

    #[test]
    fn test_prepare_cube() {
        let mut cube = Cube::new(Vec::new());

        prepare_cube(&mut cube);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OWG".to_string(), "BO".to_string(), "OGY".to_string()],
            ["GW".to_string(), "Y".to_string(), "OY".to_string()],
            ["YGR".to_string(), "RB".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    fn prepare_cube(cube: &mut Cube) {
        for _ in 0..4 {
            //R
            cube.set_front(cube.get_right());
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_left());
            //U'
            cube.set_front(cube.get_up());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_down());
            //R'
            cube.set_front(cube.get_right());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_left());
            //F
            cube.rotate_front_clockwise();
            //R
            cube.set_front(cube.get_right());
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_left());
            //U'
            cube.set_front(cube.get_up());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_down());
            //R
            cube.set_front(cube.get_right());
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_left());
            //U
            cube.set_front(cube.get_up());
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_down());
            //R
            cube.set_front(cube.get_right());
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_left());
            //U'
            cube.set_front(cube.get_up());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_down());
            //R'
            cube.set_front(cube.get_right());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_left());
            //U'
            cube.set_front(cube.get_up());
            cube.rotate_front_counterclockwise();
            cube.set_front(cube.get_down());
            //R2
            cube.set_front(cube.get_right());
            cube.rotate_front_clockwise();
            cube.rotate_front_clockwise();
            cube.set_front(cube.get_left());

            cube.set_front(cube.get_left());
        }

        //R'
        cube.set_front(cube.get_right());
        cube.rotate_front_counterclockwise();
        cube.set_front(cube.get_left());
        //F
        cube.rotate_front_clockwise();
        //B
        cube.set_front(cube.get_back());
        cube.rotate_front_clockwise();
        cube.set_front(cube.get_back());
        //L
        cube.set_front(cube.get_left());
        cube.rotate_front_clockwise();
        cube.set_front(cube.get_right());
        //B'
        cube.set_front(cube.get_back());
        cube.rotate_front_counterclockwise();
        cube.set_front(cube.get_back());
    }

    #[test]
    fn test_make_daisy() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);

        cube.make_daisy();

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["BYR".to_string(), "WG".to_string(), "BOY".to_string()],
            ["WO".to_string(), "Y".to_string(), "WB".to_string()],
            ["GWR".to_string(), "WR".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_cross_of_down_face() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();

        cube.make_cross_of_down_face();

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["WBR".to_string(), "WO".to_string(), "BYR".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["RYG".to_string(), "WR".to_string(), "OGY".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);
    }

    #[test]
    fn test_make_first_layer() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();
        cube.make_cross_of_down_face();

        cube.make_first_layer();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RBY".to_string(), "BR".to_string(), "BOY".to_string()],
            ["RG".to_string(), "O".to_string(), "BO".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);
    }

    #[test]
    fn test_make_second_layer() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();

        cube.make_second_layer();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RBY".to_string(), "YO".to_string(), "YBO".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["RYG".to_string(), "YG".to_string(), "YOG".to_string()],
            ["YR".to_string(), "Y".to_string(), "BY".to_string()],
            ["YRB".to_string(), "OY".to_string(), "BOY".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_cross_of_up_face() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();
        cube.make_second_layer();

        cube.make_cross_of_up_face();
        cube.make_right_cross_of_up_face();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["YBO".to_string(), "OY".to_string(), "GYO".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["BYR".to_string(), "YR".to_string(), "RYG".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["OYB".to_string(), "YO".to_string(), "YOG".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_corners_of_up_face() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();
        cube.make_second_layer();
        cube.make_cross_of_up_face();
        cube.make_right_cross_of_up_face();

        cube.make_corners_of_up_face();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["GYO".to_string(), "OY".to_string(), "YBO".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["GRY".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["OGY".to_string(), "YO".to_string(), "BOY".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_third_layer() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();
        cube.make_second_layer();
        cube.make_cross_of_up_face();
        cube.make_right_cross_of_up_face();
        cube.make_corners_of_up_face();

        cube.make_third_layer();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RGW".to_string(), "RW".to_string(), "RWB".to_string()],
            ["RG".to_string(), "R".to_string(), "RB".to_string()],
            ["RYG".to_string(), "RY".to_string(), "RBY".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_solve() {
        let mut cube = Cube::new(Vec::new());
        prepare_cube(&mut cube);

        cube.solve();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RGW".to_string(), "RW".to_string(), "RWB".to_string()],
            ["RG".to_string(), "R".to_string(), "RB".to_string()],
            ["RYG".to_string(), "RY".to_string(), "RBY".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_daisy_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);

        cube.make_daisy();

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["RYG".to_string(), "WB".to_string(), "WOB".to_string()],
            ["WO".to_string(), "Y".to_string(), "WR".to_string()],
            ["OYB".to_string(), "WG".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_cross_of_down_face_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);
        cube.make_daisy();

        cube.make_cross_of_down_face();

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["RGW".to_string(), "WO".to_string(), "YOG".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["YRB".to_string(), "WR".to_string(), "BRW".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);
    }

    #[test]
    fn test_make_first_layer_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);
        cube.make_daisy();
        cube.make_cross_of_down_face();

        cube.make_first_layer();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["BOY".to_string(), "OB".to_string(), "OGY".to_string()],
            ["OG".to_string(), "O".to_string(), "BR".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);
    }

    #[test]
    fn test_make_second_layer_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();

        cube.make_second_layer();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["OYB".to_string(), "RY".to_string(), "YGR".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);
    }

    #[test]
    fn test_make_cross_of_up_face_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();
        cube.make_second_layer();

        cube.make_cross_of_up_face();
        cube.make_right_cross_of_up_face();

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OYB".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["GRY".to_string(), "YO".to_string(), "YOG".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_corners_of_up_face_hard() {
        let mut cube = Cube::new(Vec::new());
        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);
        cube.make_daisy();
        cube.make_cross_of_down_face();
        cube.make_first_layer();
        cube.make_second_layer();
        cube.make_cross_of_up_face();
        cube.make_right_cross_of_up_face();

        cube.make_corners_of_up_face();

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["RYG".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["OGY".to_string(), "YO".to_string(), "YBO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_solve_hard() {
        let mut cube = Cube::new(Vec::new());

        let command = "(F'RUB'L'DFR'U'BLD')x3".to_string();
        cube.execute_command(command);

        cube.solve();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RGW".to_string(), "RW".to_string(), "RWB".to_string()],
            ["RG".to_string(), "R".to_string(), "RB".to_string()],
            ["RYG".to_string(), "RY".to_string(), "RBY".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_solve_random() {
        let orange = [
            ["RWB".to_string(), "GW".to_string(), "BYR".to_string()],
            ["BW".to_string(), "O".to_string(), "YB".to_string()],
            ["BOY".to_string(), "OB".to_string(), "OWG".to_string()],
        ];
        let red = [
            ["GWR".to_string(), "YO".to_string(), "OBW".to_string()],
            ["WR".to_string(), "R".to_string(), "RG".to_string()],
            ["GYO".to_string(), "YR".to_string(), "YGR".to_string()],
        ];
        let yellow = [
            ["BWO".to_string(), "OY".to_string(), "RGW".to_string()],
            ["YG".to_string(), "Y".to_string(), "RB".to_string()],
            ["BRW".to_string(), "WG".to_string(), "YRB".to_string()],
        ];
        let white = [
            ["OYB".to_string(), "BO".to_string(), "GOW".to_string()],
            ["OW".to_string(), "W".to_string(), "GO".to_string()],
            ["RYG".to_string(), "RY".to_string(), "YOG".to_string()],
        ];
        let green = [
            ["WOB".to_string(), "GY".to_string(), "WBR".to_string()],
            ["GR".to_string(), "G".to_string(), "WB".to_string()],
            ["GRY".to_string(), "WO".to_string(), "YBO".to_string()],
        ];
        let blue = [
            ["RBY".to_string(), "BR".to_string(), "WRG".to_string()],
            ["BY".to_string(), "B".to_string(), "RW".to_string()],
            ["WGO".to_string(), "OG".to_string(), "OGY".to_string()],
        ];

        let colors = vec![orange, red, yellow, white, green, blue];
        let cube_faces = colors
            .iter()
            .map(|color| CubeFace::new(color.clone()))
            .collect();
        let mut cube = Cube::new(cube_faces);

        cube.solve();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["GOW".to_string(), "GW".to_string(), "GWR".to_string()],
            ["GO".to_string(), "G".to_string(), "GR".to_string()],
            ["GYO".to_string(), "GY".to_string(), "GRY".to_string()],
        ];
        assert_eq!(actual_color_of_front, expected_color_of_front);

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["WOB".to_string(), "WB".to_string(), "WBR".to_string()],
            ["WO".to_string(), "W".to_string(), "WR".to_string()],
            ["WGO".to_string(), "WG".to_string(), "WRG".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }
}
