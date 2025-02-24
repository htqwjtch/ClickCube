mod cube;
use opencv::{
    core::{self, Mat, Scalar},
    imgcodecs, imgproc,
    prelude::*,
    Result,
};

fn binarize_image(input_path: &str, output_path: &str) -> Result<(), String> {
    // Загружаем изображение в градациях серого
    let mut img = imgcodecs::imread(input_path, imgcodecs::IMREAD_GRAYSCALE)
        .map_err(|e| format!("Failed to read image. Error: {}", e.to_string()))?;

    if img.empty() {
        return Err("Failed to load image".to_string());
    }

    img = match color_threshold_filter(img) {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    img = match minimal_filter(img) {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    img = match median_filter(img) {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    img = match sobel_filter(img) {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    imgcodecs::imwrite(output_path, &img, &core::Vector::new()).map_err(|e| {
        format!(
            "Failed to perform imgcodecs::imwrite. Error: {}",
            e.to_string()
        )
    })?;

    Ok(())
}

fn color_threshold_filter(img: Mat) -> Result<Mat, String> {
    let mut binary_img = Mat::default();

    imgproc::threshold(
        &img,
        &mut binary_img,
        20 as f64,
        255.0,
        imgproc::THRESH_BINARY,
    )
    .map_err(|e| {
        format!(
            "Failed to perform imgproc::threshold. Error: {}",
            e.to_string()
        )
    })?;

    Ok(binary_img)
}

fn minimal_filter(img: Mat) -> Result<Mat, String> {
    let mut filtered_img = Mat::default();

    let kernel = Mat::ones(3, 3, core::CV_8U)
        .map_err(|e| format!("Failed to create kernel. Error: {}", e.to_string()))?;
    imgproc::erode(
        &img,
        &mut filtered_img,
        &kernel,
        core::Point::new(-1, -1),
        1,
        core::BORDER_CONSTANT,
        Scalar::all(0.0),
    )
    .map_err(|e| format!("Failed to perform imgproc::erode. Error: {}", e.to_string()))?;

    Ok(filtered_img)
}

fn median_filter(img: Mat) -> Result<Mat, String> {
    let mut filtered_img = Mat::default();

    imgproc::median_blur(&img, &mut filtered_img, 3).map_err(|e| {
        format!(
            "Failed to perform imgproc::median_blur. Error: {}",
            e.to_string()
        )
    })?;

    Ok(filtered_img)
}

fn sobel_filter(img: Mat) -> Result<Mat, String> {
    let mut grad_x = Mat::default();
    let mut grad_y = Mat::default();
    let mut abs_grad_x = Mat::default();
    let mut abs_grad_y = Mat::default();
    let mut result = Mat::default();

    imgproc::sobel(
        &img,
        &mut grad_x,
        core::CV_16S,
        1,
        0,
        3,
        1.0,
        0.0,
        core::BORDER_DEFAULT,
    )
    .map_err(|e| format!("Failed to perform Sobel X. Error: {}", e.to_string()))?;
    imgproc::sobel(
        &img,
        &mut grad_y,
        core::CV_16S,
        0,
        1,
        3,
        1.0,
        0.0,
        core::BORDER_DEFAULT,
    )
    .map_err(|e| format!("Failed to perform Sobel Y. Error: {}", e.to_string()))?;

    core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)
        .map_err(|e| format!("Failed to convert scale abs X. Error: {}", e.to_string()))?;
    core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)
        .map_err(|e| format!("Failed to convert scale abs Y. Error: {}", e.to_string()))?;

    core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut result, -1).map_err(|e| {
        format!(
            "Failed to combine Sobel gradients. Error: {}",
            e.to_string()
        )
    })?;

    Ok(result)
}

fn main() -> Result<()> {
    let input_path = "assets/input.jpg";
    let output_path = "assets/output.jpg";

    binarize_image(input_path, output_path).map_err(|e| opencv::Error::new(500, e))
}
