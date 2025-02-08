use std::{cell::RefCell, rc::Rc};

pub struct Side {
    front: [[String; 3]; 3],
    back: Option<Rc<RefCell<Side>>>,
    up: Option<Rc<RefCell<Side>>>,
    down: Option<Rc<RefCell<Side>>>,
    left: Option<Rc<RefCell<Side>>>,
    right: Option<Rc<RefCell<Side>>>,
}

impl Side {
    pub fn new(front: [[String; 3]; 3]) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Side {
            front,
            back: None,
            up: None,
            down: None,
            left: None,
            right: None,
        }))
    }

    pub fn set_front(&mut self, front: [[String; 3]; 3]) {
        self.front = front;
    }

    pub fn get_front(&self) -> [[String; 3]; 3] {
        self.front.clone()
    }

    pub fn set_sides(
        &mut self,
        back: Rc<RefCell<Side>>,
        up: Rc<RefCell<Side>>,
        down: Rc<RefCell<Side>>,
        left: Rc<RefCell<Side>>,
        right: Rc<RefCell<Side>>,
    ) {
        self.back = Some(back);
        self.up = Some(up);
        self.down = Some(down);
        self.left = Some(left);
        self.right = Some(right);
    }

    pub fn clockwise(&mut self) {
        if let (Some(up), Some(left), Some(down), Some(right)) = (
            self.up.as_ref(),
            self.left.as_ref(),
            self.down.as_ref(),
            self.right.as_ref(),
        ) {
            let up_front = up.borrow().get_front();
            let left_front = left.borrow().get_front();
            let down_front = down.borrow().get_front();
            let right_front = right.borrow().get_front();

            up.borrow_mut().set_front([
                up_front[0].clone(),
                up_front[1].clone(),
                [
                    left_front[2][2].clone(),
                    left_front[1][2].clone(),
                    left_front[0][2].clone(),
                ],
            ]);
            left.borrow_mut().set_front([
                [
                    left_front[0][0].clone(),
                    left_front[0][1].clone(),
                    down_front[0][0].clone(),
                ],
                [
                    left_front[1][0].clone(),
                    left_front[1][1].clone(),
                    down_front[0][1].clone(),
                ],
                [
                    left_front[2][0].clone(),
                    left_front[2][1].clone(),
                    down_front[0][2].clone(),
                ],
            ]);
            down.borrow_mut().set_front([
                [
                    right_front[2][0].clone(),
                    right_front[1][0].clone(),
                    right_front[0][0].clone(),
                ],
                down_front[1].clone(),
                down_front[2].clone(),
            ]);
            right.borrow_mut().set_front([
                [
                    up_front[2][0].clone(),
                    right_front[0][1].clone(),
                    right_front[0][2].clone(),
                ],
                [
                    up_front[2][1].clone(),
                    right_front[1][1].clone(),
                    right_front[1][2].clone(),
                ],
                [
                    up_front[2][2].clone(),
                    right_front[2][1].clone(),
                    right_front[2][2].clone(),
                ],
            ]);
        }
        self.rotate_front_clockwise();
    }

    fn rotate_front_clockwise(&mut self) {
        let n = self.front.len();
        let mut new_front = self.front.clone();
        for i in 0..n {
            for j in 0..n {
                new_front[j][n - i - 1] = self.front[i][j].clone();
            }
        }
        self.front = new_front;
    }

    pub fn counterclockwise(&mut self) {
        for _ in 0..3 {
            self.clockwise();
        }
    }
}
