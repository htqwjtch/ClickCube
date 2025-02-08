mod side;
use side::Side;

use std::{cell::RefCell, rc::Rc};

fn main() {
    let orange = Side::new([
        ["OGY".to_string(), "OY".to_string(), "OYB".to_string()],
        ["OG".to_string(), "O".to_string(), "OB".to_string()],
        ["OWG".to_string(), "OW".to_string(), "OBW".to_string()],
    ]);
    let red = Side::new([
        ["RBY".to_string(), "RY".to_string(), "RYG".to_string()],
        ["RB".to_string(), "R".to_string(), "RG".to_string()],
        ["RWB".to_string(), "RW".to_string(), "RGW".to_string()],
    ]);
    let yellow = Side::new([
        ["YGR".to_string(), "YR".to_string(), "YRB".to_string()],
        ["YG".to_string(), "Y".to_string(), "YB".to_string()],
        ["YOG".to_string(), "YO".to_string(), "YBO".to_string()],
    ]);
    let white = Side::new([
        ["WGO".to_string(), "WO".to_string(), "WOB".to_string()],
        ["WG".to_string(), "W".to_string(), "WB".to_string()],
        ["WRG".to_string(), "WR".to_string(), "WBR".to_string()],
    ]);
    let green = Side::new([
        ["GRY".to_string(), "GY".to_string(), "GYO".to_string()],
        ["GR".to_string(), "G".to_string(), "GO".to_string()],
        ["GWR".to_string(), "GW".to_string(), "GOW".to_string()],
    ]);
    let blue = Side::new([
        ["BOY".to_string(), "BY".to_string(), "BYR".to_string()],
        ["BO".to_string(), "B".to_string(), "BR".to_string()],
        ["BWO".to_string(), "BW".to_string(), "BRW".to_string()],
    ]);

    orange.borrow_mut().set_sides(
        red.clone(),
        yellow.clone(),
        white.clone(),
        green.clone(),
        blue.clone(),
    );
    red.borrow_mut().set_sides(
        orange.clone(),
        yellow.clone(),
        white.clone(),
        blue.clone(),
        green.clone(),
    );
    yellow.borrow_mut().set_sides(
        white.clone(),
        red.clone(),
        orange.clone(),
        green.clone(),
        blue.clone(),
    );
    white.borrow_mut().set_sides(
        yellow.clone(),
        orange.clone(),
        red.clone(),
        green.clone(),
        blue.clone(),
    );
    green.borrow_mut().set_sides(
        blue.clone(),
        yellow.clone(),
        white.clone(),
        red.clone(),
        orange.clone(),
    );
    blue.borrow_mut().set_sides(
        green.clone(),
        yellow.clone(),
        white.clone(),
        orange.clone(),
        red.clone(),
    );

    println!("Setup completed!");

    orange.borrow_mut().clockwise();

    println_sides(vec![
        orange.clone(),
        red.clone(),
        yellow.clone(),
        white.clone(),
        green.clone(),
        blue.clone(),
    ]);

    orange.borrow_mut().counterclockwise();

    println_sides(vec![
        orange.clone(),
        red.clone(),
        yellow.clone(),
        white.clone(),
        green.clone(),
        blue.clone(),
    ]);
}

fn println_sides(matrixes: Vec<Rc<RefCell<Side>>>) {
    for matrix in matrixes.into_iter() {
        println!("{:?}", matrix.borrow_mut().get_front()[0]);
        println!("{:?}", matrix.borrow_mut().get_front()[1]);
        println!("{:?}", matrix.borrow_mut().get_front()[2]);
        println!("");
    }
}
