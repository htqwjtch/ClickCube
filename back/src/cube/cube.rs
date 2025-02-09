use crate::cube::side::side::Side;

use std::{cell::RefCell, rc::Rc};

pub struct Cube {
    front: Rc<RefCell<Side>>,
    back: Rc<RefCell<Side>>,
    up: Rc<RefCell<Side>>,
    down: Rc<RefCell<Side>>,
    left: Rc<RefCell<Side>>,
    right: Rc<RefCell<Side>>,
}

impl Cube {
    pub fn new() -> Self {
        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let front = Side::new(orange);
        let red = [
            ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
            ["RB".to_string(), "R".to_string(), "RG".to_string()],
            ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
        ];
        let back = Side::new(red);
        let yellow = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["YOG".to_string(), "YO".to_string(), "YBO".to_string()],
        ];
        let up = Side::new(yellow);
        let white = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        let down = Side::new(white);
        let green = [
            ["GRY".to_string(), "GY".to_string(), "GYO".to_string()],
            ["GR".to_string(), "G".to_string(), "GO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "GOW".to_string()],
        ];
        let left = Side::new(green);
        let blue = [
            ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
            ["BO".to_string(), "B".to_string(), "BR".to_string()],
            ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        let right = Side::new(blue);

        front.borrow_mut().set_adjacent_sides(
            back.clone(),
            up.clone(),
            down.clone(),
            left.clone(),
            right.clone(),
        );
        back.borrow_mut().set_adjacent_sides(
            front.clone(),
            up.clone(),
            down.clone(),
            right.clone(),
            left.clone(),
        );
        up.borrow_mut().set_adjacent_sides(
            down.clone(),
            back.clone(),
            front.clone(),
            left.clone(),
            right.clone(),
        );
        down.borrow_mut().set_adjacent_sides(
            up.clone(),
            front.clone(),
            back.clone(),
            left.clone(),
            right.clone(),
        );
        left.borrow_mut().set_adjacent_sides(
            right.clone(),
            up.clone(),
            down.clone(),
            back.clone(),
            front.clone(),
        );
        right.borrow_mut().set_adjacent_sides(
            left.clone(),
            up.clone(),
            down.clone(),
            front.clone(),
            back.clone(),
        );

        Cube {
            front: front,
            back: back,
            up: up,
            down: down,
            left: left,
            right: right,
        }
    }

    pub fn set_front(&mut self, front: Rc<RefCell<Side>>) {
        self.front = front;

        let new_back = self
            .front
            .borrow_mut()
            .get_adjacent_side("back".to_string());
        self.back = new_back.unwrap();

        let new_up = self.front.borrow_mut().get_adjacent_side("up".to_string());
        self.up = new_up.unwrap();

        let new_down = self
            .front
            .borrow_mut()
            .get_adjacent_side("down".to_string());
        self.down = new_down.unwrap();

        let new_left = self
            .front
            .borrow_mut()
            .get_adjacent_side("left".to_string());
        self.left = new_left.unwrap();

        let new_right = self
            .front
            .borrow_mut()
            .get_adjacent_side("right".to_string());
        self.right = new_right.unwrap();
    }

    pub fn get_front(&self) -> Rc<RefCell<Side>> {
        self.front.clone()
    }

    pub fn get_back(&self) -> Rc<RefCell<Side>> {
        self.back.clone()
    }

    pub fn get_up(&self) -> Rc<RefCell<Side>> {
        self.up.clone()
    }

    pub fn get_down(&self) -> Rc<RefCell<Side>> {
        self.down.clone()
    }

    pub fn get_left(&self) -> Rc<RefCell<Side>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Rc<RefCell<Side>> {
        self.right.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut cube = Cube::new();
        let actual_front = cube.get_front().borrow_mut().get_side();
        let orange = [
            ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
            ["OG".to_string(), "O".to_string(), "OB".to_string()],
            ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
        ];
        let expected_front = Side::new(orange).borrow_mut().get_side();
        assert_eq!(actual_front, expected_front);

        cube.set_front(cube.get_right());
        let actual_right = cube.get_front().borrow_mut().get_side();
        let blue = [
            ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
            ["BO".to_string(), "B".to_string(), "BR".to_string()],
            ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
        ];
        let expected_right = Side::new(blue).borrow_mut().get_side();
        assert_eq!(actual_right, expected_right);

        cube.set_front(cube.get_right());
        let actual_back = cube.get_front().borrow_mut().get_side();
        let red = [
            ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
            ["RB".to_string(), "R".to_string(), "RG".to_string()],
            ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
        ];
        let expected_back = Side::new(red).borrow_mut().get_side();
        assert_eq!(actual_back, expected_back);

        cube.set_front(cube.get_right());
        let actual_left = cube.get_front().borrow_mut().get_side();
        let green = [
            ["GRY".to_string(), "GY".to_string(), "GYO".to_string()],
            ["GR".to_string(), "G".to_string(), "GO".to_string()],
            ["GWR".to_string(), "GW".to_string(), "GOW".to_string()],
        ];
        let expected_left = Side::new(green).borrow_mut().get_side();
        assert_eq!(actual_left, expected_left);

        cube.set_front(cube.get_up());
        let actual_up = cube.get_front().borrow_mut().get_side();
        let yellow = [
            ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
            ["YG".to_string(), "Y".to_string(), "YB".to_string()],
            ["YOG".to_string(), "YO".to_string(), "YBO".to_string()],
        ];
        let expected_up = Side::new(yellow).borrow_mut().get_side();
        assert_eq!(actual_up, expected_up);

        cube.set_front(cube.get_back());
        let actual_down = cube.get_front().borrow_mut().get_side();
        let white = [
            ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
            ["WG".to_string(), "W".to_string(), "WB".to_string()],
            ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
        ];
        let expected_down = Side::new(white).borrow_mut().get_side();
        assert_eq!(actual_down, expected_down);
    }

    #[test]
    fn test_clockwise() {
        //x1, x2, x3, x4
    }

    #[test]
    fn test_counterclockwise() {
        //x1, x2, x3, x4
    }
}
