use core::time;

use crate::cube::face::face::Face;

pub struct Cube {
    front: Face,
    back: Face,
    up: Face,
    down: Face,
    left: Face,
    right: Face,
}

impl Cube {
    pub fn new(colors: Vec<[[String; 3]; 3]>) -> Self {
        let orange;
        let red;
        let yellow;
        let white;
        let green;
        let blue;
        if !colors.is_empty() {
            orange = colors[0].clone();
            red = colors[1].clone();
            yellow = colors[2].clone();
            white = colors[3].clone();
            green = colors[4].clone();
            blue = colors[5].clone();
        } else {
            orange = [
                ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
                ["OG".to_string(), "O".to_string(), "OB".to_string()],
                ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
            ];
            red = [
                ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
                ["RB".to_string(), "R".to_string(), "RG".to_string()],
                ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
            ];
            yellow = [
                ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
                ["YG".to_string(), "Y".to_string(), "YB".to_string()],
                ["YOG".to_string(), "YO".to_string(), "YBO".to_string()],
            ];
            white = [
                ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
                ["WG".to_string(), "W".to_string(), "WB".to_string()],
                ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
            ];
            green = [
                ["GRY".to_string(), "GY".to_string(), "GYO".to_string()],
                ["GR".to_string(), "G".to_string(), "GO".to_string()],
                ["GWR".to_string(), "GW".to_string(), "GOW".to_string()],
            ];
            blue = [
                ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
                ["BO".to_string(), "B".to_string(), "BR".to_string()],
                ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
            ];
        }

        Cube {
            front: Face::new(orange),
            back: Face::new(red),
            up: Face::new(yellow),
            down: Face::new(white),
            left: Face::new(green),
            right: Face::new(blue),
        }
    }

    pub fn set_front(&mut self, front: Face) {
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

    pub fn get_front(&self) -> Face {
        self.front.clone()
    }

    fn set_back(&mut self, back: Face) {
        self.back = back;
    }

    pub fn get_back(&self) -> Face {
        self.back.clone()
    }

    fn set_up(&mut self, up: Face) {
        self.up = up;
    }

    pub fn get_up(&self) -> Face {
        self.up.clone()
    }

    fn set_down(&mut self, down: Face) {
        self.down = down;
    }

    pub fn get_down(&self) -> Face {
        self.down.clone()
    }

    fn set_left(&mut self, left: Face) {
        self.left = left;
    }

    pub fn get_left(&self) -> Face {
        self.left.clone()
    }

    fn set_right(&mut self, right: Face) {
        self.right = right;
    }

    pub fn get_right(&self) -> Face {
        self.right.clone()
    }

    pub fn rotate_front_clockwise(&mut self) {
        let color_of_up = self.up.get_color();
        let color_of_left = self.left.get_color();
        let color_of_down = self.down.get_color();
        let color_of_right = self.right.get_color();

        let new_color_of_up = [
            color_of_up[0].clone(),
            color_of_up[1].clone(),
            [
                color_of_left[2][2].clone(),
                color_of_left[1][2].clone(),
                color_of_left[0][2].clone(),
            ],
        ];
        self.set_up(Face::new(new_color_of_up));

        let new_color_of_left = [
            [
                color_of_left[0][0].clone(),
                color_of_left[0][1].clone(),
                color_of_down[0][0].clone(),
            ],
            [
                color_of_left[1][0].clone(),
                color_of_left[1][1].clone(),
                color_of_down[0][1].clone(),
            ],
            [
                color_of_left[2][0].clone(),
                color_of_left[2][1].clone(),
                color_of_down[0][2].clone(),
            ],
        ];
        self.set_left(Face::new(new_color_of_left));

        let new_color_of_down = [
            [
                color_of_right[2][0].clone(),
                color_of_right[1][0].clone(),
                color_of_right[0][0].clone(),
            ],
            color_of_down[1].clone(),
            color_of_down[2].clone(),
        ];
        self.set_down(Face::new(new_color_of_down));

        let new_color_of_right = [
            [
                color_of_up[2][0].clone(),
                color_of_right[0][1].clone(),
                color_of_right[0][2].clone(),
            ],
            [
                color_of_up[2][1].clone(),
                color_of_right[1][1].clone(),
                color_of_right[1][2].clone(),
            ],
            [
                color_of_up[2][2].clone(),
                color_of_right[2][1].clone(),
                color_of_right[2][2].clone(),
            ],
        ];
        self.set_right(Face::new(new_color_of_right));

        self.set_front(self.rotate_face_clockwise(self.front.clone()));
    }

    fn rotate_face_clockwise(&self, face: Face) -> Face {
        let mut new_color = face.get_color().clone();
        let n = new_color.len();
        for i in 0..n {
            for j in 0..n {
                new_color[j][n - i - 1] = face.get_color()[i][j].clone();
            }
        }
        Face::new(new_color)
    }

    pub fn rotate_front_counterclockwise(&mut self) {
        for _ in 0..3 {
            self.rotate_front_clockwise();
        }
    }

    fn rotate_face_counterclockwise(&self, face: Face) -> Face {
        let mut new_face = face;
        for _ in 0..3 {
            new_face = self.rotate_face_clockwise(new_face.clone());
        }
        new_face
    }

    pub fn pif_paf_right(&mut self) {
        self.set_front(self.get_right());
        self.rotate_front_clockwise();
        self.set_front(self.get_up());
        self.rotate_front_clockwise();
        self.set_front(self.get_down());
        self.rotate_front_counterclockwise();
        self.set_front(self.get_up());
        self.rotate_front_counterclockwise();
        self.set_front(self.get_down());
        self.set_front(self.get_left());
    }

    pub fn pif_paf_left(&mut self) {
        self.set_front(self.get_left());
        self.rotate_front_counterclockwise();
        self.set_front(self.get_up());
        self.rotate_front_counterclockwise();
        self.set_front(self.get_down());
        self.rotate_front_clockwise();
        self.set_front(self.get_up());
        self.rotate_front_clockwise();
        self.set_front(self.get_down());
        self.set_front(self.get_right());
    }

    pub fn solve(&mut self) {
        //what if some steps are already ready
        self.make_daisy();
        self.make_cross_of_down();
        self.make_corners_of_down();
        self.make_second_layer();
        self.make_cross_of_up();
        self.make_corners_of_up();
        self.make_third_layer();
    }

    fn make_daisy(&mut self) {
        for _ in 0..4 {
            self.lift_edge_of_down();
        }
    }

    fn lift_edge_of_down(&mut self) {
        let mut command_to_lift_edge = self.check_edges_of_down();
        if !command_to_lift_edge.is_empty() {
            self.execute_command(command_to_lift_edge + "Y'");
            return;
        }

        command_to_lift_edge = self.check_edges_of_first_layer();
        if !command_to_lift_edge.is_empty() {
            self.execute_command(command_to_lift_edge + "Y'");
            return;
        }

        command_to_lift_edge = self.check_edges_of_second_layer();
        if !command_to_lift_edge.is_empty() {
            self.execute_command(command_to_lift_edge + "Y'");
            return;
        }

        command_to_lift_edge = self.check_edges_of_third_layer();
        if !command_to_lift_edge.is_empty() {
            self.execute_command(command_to_lift_edge + "Y'");
            return;
        }
    }

    fn check_edges_of_down(&mut self) -> String {
        let mut command_to_lift_edge = String::new();

        let color_of_up = self.get_up().get_color();
        let color_of_down = self.get_down().get_color();

        let center_color_of_down = &self.get_down().get_color()[1][1][0..1];

        if &color_of_down[0][1][0..1] == center_color_of_down {
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F2";
        } else if &color_of_down[1][0][0..1] == center_color_of_down {
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L2";
        } else if &color_of_down[2][1][0..1] == center_color_of_down {
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B2";
        } else if &color_of_down[1][2][0..1] == center_color_of_down {
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R2";
        }

        command_to_lift_edge
    }

    fn check_edges_of_first_layer(&mut self) -> String {
        let mut command_to_lift_edge = String::new();

        let color_of_front = self.get_front().get_color();
        let color_of_back = self.get_back().get_color();
        let color_of_up = self.get_up().get_color();
        let color_of_left = self.get_left().get_color();
        let color_of_right = self.get_right().get_color();

        let center_color_of_down = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front[2][1][0..1] == center_color_of_down {
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F";
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L'";
        } else if &color_of_left[2][1][0..1] == center_color_of_down {
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L";
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B'";
        } else if &color_of_back[2][1][0..1] == center_color_of_down {
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B";
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R'";
        } else if &color_of_right[2][1][0..1] == center_color_of_down {
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R";
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F'";
        }

        command_to_lift_edge
    }

    fn check_edges_of_second_layer(&mut self) -> String {
        let mut command_to_lift_edge = String::new();

        let color_of_front = self.get_front().get_color();
        let color_of_back = self.get_back().get_color();
        let color_of_up = self.get_up().get_color();
        let color_of_left = self.get_left().get_color();
        let color_of_right = self.get_right().get_color();

        let center_color_of_down = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front[1][0][0..1] == center_color_of_down {
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L'";
        } else if &color_of_front[1][2][0..1] == center_color_of_down {
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R";
        } else if &color_of_left[1][0][0..1] == center_color_of_down {
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B'";
        } else if &color_of_left[1][2][0..1] == center_color_of_down {
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F";
        } else if &color_of_back[1][0][0..1] == center_color_of_down {
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R'";
        } else if &color_of_back[1][2][0..1] == center_color_of_down {
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L";
        } else if &color_of_right[1][0][0..1] == center_color_of_down {
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F'";
        } else if &color_of_right[1][2][0..1] == center_color_of_down {
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B";
        }

        command_to_lift_edge
    }

    fn check_edges_of_third_layer(&mut self) -> String {
        let mut command_to_lift_edge = String::new();

        let color_of_front = self.get_front().get_color();
        let color_of_back = self.get_back().get_color();
        let color_of_up = self.get_up().get_color();
        let color_of_left = self.get_left().get_color();
        let color_of_right = self.get_right().get_color();

        let center_color_of_down = &self.get_down().get_color()[1][1][0..1];

        if &color_of_front[0][1][0..1] == center_color_of_down {
            command_to_lift_edge += "F'";
            if &color_of_up[1][0][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[2][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][2][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "L'";
        } else if &color_of_left[0][1][0..1] == center_color_of_down {
            command_to_lift_edge += "L'";
            if &color_of_up[0][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][0][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[2][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "B'";
        } else if &color_of_back[0][1][0..1] == center_color_of_down {
            command_to_lift_edge += "B'";
            if &color_of_up[1][2][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[0][1][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[1][0][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "R'";
        } else if &color_of_right[0][1][0..1] == center_color_of_down {
            command_to_lift_edge += "R'";
            if &color_of_up[2][1][0..1] == center_color_of_down {
                command_to_lift_edge += "U";
                let mut times = 1;
                if &color_of_up[1][2][0..1] == center_color_of_down {
                    times += 1;
                    if &color_of_up[0][1][0..1] == center_color_of_down {
                        times = 1;
                        command_to_lift_edge += "'";
                    }
                }
                if times > 1 {
                    command_to_lift_edge += times.to_string().as_str();
                }
            }
            command_to_lift_edge += "F'";
        }

        command_to_lift_edge
    }

    fn execute_command(&mut self, command: String) {
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
                    self.set_front(self.get_down());
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

    fn make_cross_of_down(&mut self) {
        for _ in 0..4 {
            for _ in 0..4 {
                //TO CHECK MOMENT WHEN ARE SOME FACES ALREADY READY
                self.lower_edge_of_up();
                self.set_front(self.get_left());
            }
            let command_to_lower_lower_edge_of_up =
                self.check_same_center_of_front_and_upper_edge_of_front();
            self.execute_command(command_to_lower_lower_edge_of_up + "Y'");
        }
    }

    fn lower_edge_of_up(&mut self) {
        if self.get_up().get_color()[2][1][0..1] == self.get_down().get_color()[1][1][0..1]
            && self.get_front().get_color()[0][1][0..1] == self.get_front().get_color()[1][1][0..1]
        {
            self.rotate_front_clockwise();
            self.rotate_front_clockwise();
        }
    }

    fn check_same_center_of_front_and_upper_edge_of_front(&mut self) -> String {
        let mut command_to_lower = String::new();

        let color_of_back = self.get_back().get_color();
        let color_of_left = self.get_left().get_color();
        let color_of_right = self.get_right().get_color();

        let center_color_of_front = &self.get_front().get_color()[1][1][0..1];
        let center_color_of_down = &self.get_down().get_color()[1][1][0..1];

        if &self.get_up().get_color()[1][1][0..1] == center_color_of_down
            && &color_of_right[0][1][0..1] == center_color_of_front
        {
            command_to_lower += "UF2";
        } else if &self.get_up().get_color()[0][1][0..1] == center_color_of_down
            && &color_of_back[0][1][0..1] == center_color_of_front
        {
            command_to_lower += "U2F2";
        } else if &self.get_up().get_color()[1][0][0..1] == center_color_of_down
            && &color_of_left[0][1][0..1] == center_color_of_front
        {
            command_to_lower += "U'F2";
        }
        command_to_lower
    }

    fn make_corners_of_down(&mut self) {
        self.lift_corners_of_down();
        self.lower_corners_of_down();
    }

    fn lift_corners_of_down(&mut self) {
        for _ in 0..2 {
            for _ in 0..2 {
                let command_to_lift_corner = self.check_corners_of_down();
                if !command_to_lift_corner.is_empty() {
                    self.execute_command(command_to_lift_corner);
                }
            }
            self.execute_command("Y2".to_string());
        }
    }

    fn check_corners_of_down(&mut self) -> String {
        let mut command_to_lift = String::new();

        let color_of_upper_left_corner_of_down = self.get_down().get_color()[0][0].clone();
        let color_of_upper_right_corner_of_down = self.get_down().get_color()[0][2].clone();

        let color_of_upper_left_corner_of_up = self.get_up().get_color()[0][0].clone();
        let color_of_upper_right_corner_of_up = self.get_up().get_color()[0][2].clone();
        let color_of_lower_left_corner_of_up = self.get_up().get_color()[2][0].clone();
        let color_of_lower_right_corner_of_up = self.get_up().get_color()[2][2].clone();

        let center_color_of_front = &self.get_front().get_color()[1][1][0..1];
        let center_color_of_left = &self.get_left().get_color()[1][1][0..1];
        let center_color_of_right = &self.get_right().get_color()[1][1][0..1];

        if color_of_upper_left_corner_of_down.contains("W")
            && ((&color_of_upper_left_corner_of_down[0..1] != "W")
                || (&color_of_upper_left_corner_of_down[0..1] == "W"
                    && (!color_of_upper_left_corner_of_down.contains(center_color_of_left)
                        || !color_of_upper_left_corner_of_down.contains(center_color_of_front))))
        {
            if color_of_lower_left_corner_of_up.contains("W") {
                if !color_of_lower_right_corner_of_up.contains("W") {
                    command_to_lift += "U";
                } else if !color_of_upper_left_corner_of_up.contains("W") {
                    command_to_lift += "U'";
                } else if !color_of_upper_right_corner_of_up.contains("W") {
                    command_to_lift += "U2";
                }
            }
            command_to_lift += "L'U'LU";
        } else if color_of_upper_right_corner_of_down.contains("W")
            && (&color_of_upper_right_corner_of_down[0..1] != "W"
                || (&color_of_upper_right_corner_of_down[0..1] == "W"
                    && (!color_of_upper_right_corner_of_down.contains(center_color_of_front)
                        || !color_of_upper_right_corner_of_down.contains(center_color_of_right))))
        {
            if color_of_lower_right_corner_of_up.contains("W") {
                if !color_of_upper_right_corner_of_up.contains("W") {
                    command_to_lift += "U";
                } else if !color_of_lower_left_corner_of_up.contains("W") {
                    command_to_lift += "U'";
                } else if !color_of_upper_left_corner_of_up.contains("W") {
                    command_to_lift += "U2";
                }
            }
            command_to_lift += "RUR'U'";
        }

        command_to_lift
    }

    fn lower_corners_of_down(&mut self) {
        for _ in 0..4 {
            for _ in 0..2 {
                let command_to_lift_corner = self.check_corners_of_up();
                if !command_to_lift_corner.is_empty() {
                    self.execute_command(command_to_lift_corner);
                }
            }
            self.execute_command("Y'".to_string());
        }
    }

    fn check_corners_of_up(&mut self) -> String {
        let mut command_to_lower = String::new();

        let color_of_upper_left_corner_of_up = self.get_up().get_color()[0][0].clone();
        let color_of_upper_right_corner_of_up = self.get_up().get_color()[0][2].clone();
        let color_of_lower_left_corner_of_up = self.get_up().get_color()[2][0].clone();
        let color_of_lower_right_corner_of_up = self.get_up().get_color()[2][2].clone();

        let second_color_of_upper_left_corner_of_down = &self.get_left().get_color()[1][1][0..1];
        let third_color_of_upper_left_corner_of_down = &self.get_front().get_color()[1][1][0..1];

        let second_color_of_upper_right_corner_of_down = &self.get_front().get_color()[1][1][0..1];
        let third_color_of_upper_right_corner_of_down = &self.get_right().get_color()[1][1][0..1];

        if color_of_lower_left_corner_of_up.contains(second_color_of_upper_left_corner_of_down)
            && color_of_lower_left_corner_of_up.contains(third_color_of_upper_left_corner_of_down)
        {
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(L'U'LU)x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "Y'RUR'U'Y";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "L'U'LU";
            }
        } else if color_of_lower_right_corner_of_up
            .contains(second_color_of_upper_right_corner_of_down)
            && color_of_lower_right_corner_of_up.contains(third_color_of_upper_right_corner_of_down)
        {
            if &color_of_lower_right_corner_of_up[0..1] == "W" {
                command_to_lower += "(RUR'U')x3";
            } else if &color_of_lower_right_corner_of_up[1..2] == "W" {
                command_to_lower += "RUR'U'";
            } else if &color_of_lower_right_corner_of_up[2..3] == "W" {
                command_to_lower += "YL'U'LUY'";
            }
        } else if color_of_upper_left_corner_of_up
            .contains(second_color_of_upper_left_corner_of_down)
            && color_of_upper_left_corner_of_up.contains(third_color_of_upper_left_corner_of_down)
        {
            command_to_lower += "U'";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(L'U'LU)x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "Y'RUR'U'Y";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "L'U'LU";
            }
        } else if color_of_lower_right_corner_of_up
            .contains(second_color_of_upper_left_corner_of_down)
            && color_of_lower_right_corner_of_up.contains(third_color_of_upper_left_corner_of_down)
        {
            command_to_lower += "U";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(L'U'LU)x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "Y'RUR'U'Y";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "L'U'LU";
            }
        } else if color_of_lower_left_corner_of_up
            .contains(second_color_of_upper_right_corner_of_down)
            && color_of_lower_left_corner_of_up.contains(third_color_of_upper_right_corner_of_down)
        {
            command_to_lower += "U'";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(RUR'U')x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "RUR'U'";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "YL'U'LUY'";
            }
        } else if color_of_upper_right_corner_of_up
            .contains(second_color_of_upper_right_corner_of_down)
            && color_of_upper_right_corner_of_up.contains(third_color_of_upper_right_corner_of_down)
        {
            command_to_lower += "U";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(RUR'U')x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "RUR'U'";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "YL'U'LUY'";
            }
        } else if color_of_upper_right_corner_of_up
            .contains(second_color_of_upper_left_corner_of_down)
            && color_of_upper_right_corner_of_up.contains(third_color_of_upper_left_corner_of_down)
        {
            command_to_lower += "U2";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(L'U'LU)x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "Y'RUR'U'Y";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "L'U'LU";
            }
        } else if color_of_upper_left_corner_of_up
            .contains(second_color_of_upper_right_corner_of_down)
            && color_of_upper_left_corner_of_up.contains(third_color_of_upper_right_corner_of_down)
        {
            command_to_lower += "U2";
            if &color_of_lower_left_corner_of_up[0..1] == "W" {
                command_to_lower += "(RUR'U')x3";
            } else if &color_of_lower_left_corner_of_up[1..2] == "W" {
                command_to_lower += "RUR'U'";
            } else if &color_of_lower_left_corner_of_up[2..3] == "W" {
                command_to_lower += "YL'U'LUY'";
            }
        }
        command_to_lower
    }

    fn make_second_layer(&mut self) {}

    fn make_cross_of_up(&mut self) {}

    fn make_corners_of_up(&mut self) {}

    fn make_third_layer(&mut self) {}
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
        let expected_front = Face::new(orange).get_color();
        assert_eq!(actual_front, expected_front);

        cube.set_front(cube.get_right());
        let actual_right = cube.get_front().get_color();
        let blue = [
            ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
            ["BO".to_string(), "B".to_string(), "BR".to_string()],
            ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        let expected_right = Face::new(blue).get_color();
        assert_eq!(actual_right, expected_right);

        cube.set_front(cube.get_down());
        let actual_back = cube.get_front().get_color();
        let red = [
            ["WOB".to_string(), "WB".to_string(), "WBR".to_string()],
            ["WO".to_string(), "W".to_string(), "WR".to_string()],
            ["WGO".to_string(), "WG".to_string(), "WRG".to_string()],
        ];
        let expected_back = Face::new(red).get_color();
        assert_eq!(actual_back, expected_back);

        cube.set_front(cube.get_back());
        let actual_left = cube.get_front().get_color();
        let green = [
            ["YRB".to_string(), "YB".to_string(), "YBO".to_string()],
            ["YR".to_string(), "Y".to_string(), "YO".to_string()],
            ["YGR".to_string(), "YG".to_string(), "YOG".to_string()],
        ];
        let expected_left = Face::new(green).get_color();
        assert_eq!(actual_left, expected_left);

        cube.set_front(cube.get_back());
        let actual_back = cube.get_front().get_color();
        let red = [
            ["WOB".to_string(), "WB".to_string(), "WBR".to_string()],
            ["WO".to_string(), "W".to_string(), "WR".to_string()],
            ["WGO".to_string(), "WG".to_string(), "WRG".to_string()],
        ];
        let expected_back = Face::new(red).get_color();
        assert_eq!(actual_back, expected_back);

        cube.set_front(cube.get_back());
        let actual_left = cube.get_front().get_color();
        let green = [
            ["YRB".to_string(), "YB".to_string(), "YBO".to_string()],
            ["YR".to_string(), "Y".to_string(), "YO".to_string()],
            ["YGR".to_string(), "YG".to_string(), "YOG".to_string()],
        ];
        let expected_left = Face::new(green).get_color();
        assert_eq!(actual_left, expected_left);

        cube.set_front(cube.get_up());
        let actual_up = cube.get_front().get_color();
        let yellow = [
            ["BRW".to_string(), "BW".to_string(), "BWO".to_string()],
            ["BR".to_string(), "B".to_string(), "BO".to_string()],
            ["BYR".to_string(), "BY".to_string(), "BOY".to_string()],
        ];
        let expected_up = Face::new(yellow).get_color();
        assert_eq!(actual_up, expected_up);

        cube.set_front(cube.get_left());
        let actual_down = cube.get_front().get_color();
        let white = [
            ["RGW".to_string(), "RW".to_string(), "RWB".to_string()],
            ["RG".to_string(), "R".to_string(), "RB".to_string()],
            ["RYG".to_string(), "RY".to_string(), "RBY".to_string()],
        ];
        let expected_down = Face::new(white).get_color();
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
    fn test_pif_paf_right_x1_x6() {
        let mut cube = Cube::new(Vec::new());
        let expected_front_x0 = cube.get_front().get_color();
        let expected_back_x0 = cube.get_back().get_color();
        let expected_up_x0 = cube.get_up().get_color();
        let expected_down_x0 = cube.get_down().get_color();
        let expected_left_x0 = cube.get_left().get_color();
        let expected_right_x0 = cube.get_right().get_color();
        cube.pif_paf_right();
        let expected_front_x1 = [
            ["OGY".to_string(), "OY".to_string(), "WOB".to_string()],
            ["OG".to_string(), "O".to_string(), "YR".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OYB".to_string()],
        ];
        assert_eq!(cube.get_front().get_color(), expected_front_x1);
        for _ in 0..5 {
            cube.pif_paf_right();
        }
        let actual_front_x6 = cube.get_front().get_color();
        assert_eq!(actual_front_x6, expected_front_x0);
        let actual_back_x6 = cube.get_back().get_color();
        assert_eq!(actual_back_x6, expected_back_x0);
        let actual_up_x6 = cube.get_up().get_color();
        assert_eq!(actual_up_x6, expected_up_x0);
        let actual_down_x6 = cube.get_down().get_color();
        assert_eq!(actual_down_x6, expected_down_x0);
        let actual_left_x6 = cube.get_left().get_color();
        assert_eq!(actual_left_x6, expected_left_x0);
        let actual_right_x6 = cube.get_right().get_color();
        assert_eq!(actual_right_x6, expected_right_x0);
    }

    #[test]
    fn test_pif_paf_left_x1_x6() {
        let mut cube = Cube::new(Vec::new());
        let expected_front_x0 = cube.get_front().get_color();
        let expected_back_x0 = cube.get_back().get_color();
        let expected_up_x0 = cube.get_up().get_color();
        let expected_down_x0 = cube.get_down().get_color();
        let expected_left_x0 = cube.get_left().get_color();
        let expected_right_x0 = cube.get_right().get_color();
        cube.pif_paf_left();
        let expected_front_x1 = [
            ["WGO".to_string(), "OY".to_string(), "OYB".to_string()],
            ["YR".to_string(), "O".to_string(), "OB".to_string()],
            ["OGY".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(cube.get_front().get_color(), expected_front_x1);
        for _ in 0..5 {
            cube.pif_paf_left();
        }
        let actual_front_x6 = cube.get_front().get_color();
        assert_eq!(actual_front_x6, expected_front_x0);
        let actual_back_x6 = cube.get_back().get_color();
        assert_eq!(actual_back_x6, expected_back_x0);
        let actual_up_x6 = cube.get_up().get_color();
        assert_eq!(actual_up_x6, expected_up_x0);
        let actual_down_x6 = cube.get_down().get_color();
        assert_eq!(actual_down_x6, expected_down_x0);
        let actual_left_x6 = cube.get_left().get_color();
        assert_eq!(actual_left_x6, expected_left_x0);
        let actual_right_x6 = cube.get_right().get_color();
        assert_eq!(actual_right_x6, expected_right_x0);
    }

    #[test]
    fn test_make_daisy_all_edges_of_down_on_down() {
        let mut cube = Cube::new(Vec::new());
        cube.make_daisy();
        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["WBR".to_string(), "WR".to_string(), "YOG".to_string()],
            ["WG".to_string(), "Y".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WO".to_string(), "YBO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);
    }

    #[test]
    fn test_make_daisy_hard() {
        let mut cube = Cube::new(Vec::new());
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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OGY".to_string(), "WG".to_string(), "WBR".to_string()],
            ["YG".to_string(), "Y".to_string(), "WR".to_string()],
            ["OBW".to_string(), "WB".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OWG".to_string(), "BO".to_string(), "OGY".to_string()],
            ["GW".to_string(), "Y".to_string(), "OY".to_string()],
            ["YGR".to_string(), "RB".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

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
    fn test_make_cross_of_down() {
        let mut cube = Cube::new(Vec::new());
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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OGY".to_string(), "WG".to_string(), "WBR".to_string()],
            ["YG".to_string(), "Y".to_string(), "WR".to_string()],
            ["OBW".to_string(), "WB".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OWG".to_string(), "BO".to_string(), "OGY".to_string()],
            ["GW".to_string(), "Y".to_string(), "OY".to_string()],
            ["YGR".to_string(), "RB".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

        cube.make_daisy();
        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["BYR".to_string(), "WG".to_string(), "BOY".to_string()],
            ["WO".to_string(), "Y".to_string(), "WB".to_string()],
            ["GWR".to_string(), "WR".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

        cube.make_cross_of_down();

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["WBR".to_string(), "WO".to_string(), "BYR".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["RYG".to_string(), "WR".to_string(), "OGY".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);
    }

    #[test]
    fn test_make_corners_of_down() {
        let mut cube = Cube::new(Vec::new());
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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OGY".to_string(), "WG".to_string(), "WBR".to_string()],
            ["YG".to_string(), "Y".to_string(), "WR".to_string()],
            ["OBW".to_string(), "WB".to_string(), "WGO".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

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

        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["OWG".to_string(), "BO".to_string(), "OGY".to_string()],
            ["GW".to_string(), "Y".to_string(), "OY".to_string()],
            ["YGR".to_string(), "RB".to_string(), "WOB".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

        cube.make_daisy();
        let actual_color_of_up = cube.get_up().get_color();
        let expected_color_of_up = [
            ["BYR".to_string(), "WG".to_string(), "BOY".to_string()],
            ["WO".to_string(), "Y".to_string(), "WB".to_string()],
            ["GWR".to_string(), "WR".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_up, expected_color_of_up);

        cube.make_cross_of_down();

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["WBR".to_string(), "WO".to_string(), "BYR".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["RYG".to_string(), "WR".to_string(), "OGY".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);

        cube.make_corners_of_down();

        let actual_color_of_front = cube.get_front().get_color();
        let expected_color_of_front = [
            ["RYG".to_string(), "RY".to_string(), "BYR".to_string()],
            ["RG".to_string(), "O".to_string(), "BO".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);

        let actual_color_of_down = cube.get_down().get_color();
        let expected_color_of_down = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        assert_eq!(actual_color_of_down, expected_color_of_down);
    }
}
