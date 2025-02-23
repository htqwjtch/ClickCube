mod cube;

use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::path::Path;

fn main() -> opencv::Result<()> {
    let img_path = Path::new("assets/orange.jpg");

    let img = imgcodecs::imread(img_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    imgcodecs::imwrite("assets/gray.jpg", &gray, &core::Vector::new())?;

    println!("Ok! The image has been saved in assets/gray.jpg");

    while true {}
    Ok(())
}