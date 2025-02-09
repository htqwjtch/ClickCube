use std::{cell::RefCell, rc::Rc};

pub struct Side {
    side: [[String; 3]; 3],
    back: Option<Rc<RefCell<Side>>>,
    up: Option<Rc<RefCell<Side>>>,
    down: Option<Rc<RefCell<Side>>>,
    left: Option<Rc<RefCell<Side>>>,
    right: Option<Rc<RefCell<Side>>>,
}

impl Side {
    pub fn new(side: [[String; 3]; 3]) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Side {
            side,
            back: None,
            up: None,
            down: None,
            left: None,
            right: None,
        }))
    }

    pub fn set_side(&mut self, side: [[String; 3]; 3]) {
        self.side = side;
    }

    pub fn get_side(&self) -> [[String; 3]; 3] {
        self.side.clone()
    }

    pub fn set_adjacent_sides(
        &mut self,
        back: Rc<RefCell<Side>>,
        up: Rc<RefCell<Side>>,
        down: Rc<RefCell<Side>>,
        left: Rc<RefCell<Side>>,
        right: Rc<RefCell<Side>>,
    ) {
        self.set_adjacent_side("back".to_string(), back);
        self.set_adjacent_side("up".to_string(), up);
        self.set_adjacent_side("down".to_string(), down);
        self.set_adjacent_side("left".to_string(), left);
        self.set_adjacent_side("right".to_string(), right);
    }

    fn set_adjacent_side(&mut self, side_name: String, side: Rc<RefCell<Side>>) {
        match side_name.as_str() {
            "back" => self.back = Some(side),
            "up" => self.up = Some(side),
            "down" => self.down = Some(side),
            "left" => self.left = Some(side),
            "right" => self.right = Some(side),
            _ => println!("Unknown side name"),
        };
    }

    pub fn get_adjacent_side(&mut self, side_name: String) -> Option<Rc<RefCell<Side>>> {
        match side_name.as_str() {
            "back" => self.back.clone(),
            "up" => self.up.clone(),
            "down" => self.down.clone(),
            "left" => self.left.clone(),
            "right" => self.right.clone(),
            _ => None,
        }
    }

    pub fn get_back(&self) -> Option<Rc<RefCell<Side>>> {
        self.back.clone()
    }

    pub fn get_up(&self) -> Option<Rc<RefCell<Side>>> {
        self.up.clone()
    }

    pub fn get_down(&self) -> Option<Rc<RefCell<Side>>> {
        self.down.clone()
    }

    pub fn get_left(&self) -> Option<Rc<RefCell<Side>>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<Side>>> {
        self.right.clone()
    }

    pub fn clockwise(&mut self) {
        if let (Some(up), Some(left), Some(down), Some(right)) = (
            self.up.as_ref(),
            self.left.as_ref(),
            self.down.as_ref(),
            self.right.as_ref(),
        ) {
            let up_side = up.borrow().get_side();
            let left_side = left.borrow().get_side();
            let down_side = down.borrow().get_side();
            let right_side = right.borrow().get_side();

            up.borrow_mut().set_side([
                up_side[0].clone(),
                up_side[1].clone(),
                [
                    left_side[2][2].clone(),
                    left_side[1][2].clone(),
                    left_side[0][2].clone(),
                ],
            ]);
            left.borrow_mut().set_side([
                [
                    left_side[0][0].clone(),
                    left_side[0][1].clone(),
                    down_side[0][0].clone(),
                ],
                [
                    left_side[1][0].clone(),
                    left_side[1][1].clone(),
                    down_side[0][1].clone(),
                ],
                [
                    left_side[2][0].clone(),
                    left_side[2][1].clone(),
                    down_side[0][2].clone(),
                ],
            ]);
            down.borrow_mut().set_side([
                [
                    right_side[2][0].clone(),
                    right_side[1][0].clone(),
                    right_side[0][0].clone(),
                ],
                down_side[1].clone(),
                down_side[2].clone(),
            ]);
            right.borrow_mut().set_side([
                [
                    up_side[2][0].clone(),
                    right_side[0][1].clone(),
                    right_side[0][2].clone(),
                ],
                [
                    up_side[2][1].clone(),
                    right_side[1][1].clone(),
                    right_side[1][2].clone(),
                ],
                [
                    up_side[2][2].clone(),
                    right_side[2][1].clone(),
                    right_side[2][2].clone(),
                ],
            ]);
        }
        self.rotate_side_clockwise();
    }

    fn rotate_side_clockwise(&mut self) {
        let n = self.side.len();
        let mut new_side = self.side.clone();
        for i in 0..n {
            for j in 0..n {
                new_side[j][n - i - 1] = self.side[i][j].clone();
            }
        }
        self.side = new_side;
    }

    pub fn counterclockwise(&mut self) {
        for _ in 0..3 {
            self.clockwise();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let correct_orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let actual_side = Side::new(correct_orange.clone());

        let expected_side = correct_orange;

        assert_eq!(actual_side.borrow_mut().get_side(), expected_side);

        let incorrect_orange = [
            ["OGY".to_string(), "Oy".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let actual_side = Side::new(incorrect_orange);

        assert_ne!(actual_side.borrow_mut().get_side(), expected_side);
    }

    #[test]
    fn test_set_side() {
        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let actual_side = Side::new(orange);

        let red = [
            ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
            ["RB".to_string(), "R".to_string(), "RG".to_string()],
            ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
        ];
        actual_side.borrow_mut().set_side(red.clone());

        let expected_side = red;

        assert_eq!(actual_side.borrow_mut().get_side(), expected_side);

        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let expected_side = orange;

        assert_ne!(actual_side.borrow_mut().get_side(), expected_side);
    }
}
