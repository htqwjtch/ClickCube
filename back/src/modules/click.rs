use image::{GenericImageView, ImageReader, DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::path::Path;

const CENTER_COLORS: [&str; 6] = ["O", "R", "Y", "W", "G", "B"];

fn open_image(image_path: &str) -> DynamicImage {
    let reader = ImageReader::open(image_path)
        .expect("Failed to open file")
        .with_guessed_format()
        .expect("Failed to guess image format");
    reader.decode().expect("Failed to decode image")
}

fn ensure_jpeg_format(image_path: &str) -> String {
    let path = Path::new(image_path);
    
    if let Some(ext) = path.extension() {
        if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {
            return image_path.to_string();
        }
    }

    let img = open_image(image_path);
    let new_path = format!("{}.jpg", path.with_extension("").display());

    img.to_rgb8()
        .save_with_format(&new_path, ImageFormat::Jpeg)
        .expect("Failed to save as JPEG");
    
    new_path
}



fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    let mut h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    if h < 0.0 {
        h += 360.0;
    }
    
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;
    
    ((h / 360.0 * 179.0) as u8, (s * 255.0) as u8, (v * 255.0) as u8)
}

fn process_image(image_path: &str) -> Vec<(u8, u8, u8)> {
    let image_path = ensure_jpeg_format(image_path);
    let img = open_image(&image_path);
    let (width, height) = img.dimensions();

    let part_width = width / 3;
    let part_height = height / 3;
    let mut hsv_vector = Vec::new();

    for row in 0..3 {
        for col in 0..3 {
            let mut counter = HashMap::new();
            for y in (row * part_height)..((row + 1) * part_height) {
                for x in (col * part_width)..((col + 1) * part_width) {
                    let pixel = img.get_pixel(x, y);
                    let (h, s, v) = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
                    *counter.entry((h, s, v)).or_insert(0) += 1;
                }
            }
            let most_common = counter.iter().max_by_key(|&(_, &count)| count).unwrap().0;
            hsv_vector.push(*most_common);
        }
    }
    hsv_vector
}

pub fn detect_colors() -> Vec<Vec<String>> {
    let folder_path = "./assets";
    let mut hsv_vectors = Vec::new();
    let mut fifth_area_colors = Vec::new();

    for i in 1..=6 {
        let image_name = format!("cube{}.jpg", i);
        let image_path = format!("{}/{}", folder_path, image_name);
        if Path::new(&image_path).exists() {
            let hsv_vector = process_image(&image_path);
            hsv_vectors.push(hsv_vector.clone());
            fifth_area_colors.push(hsv_vector[4]);
        } else {
            println!("File {} not found.", image_name);
        }
    }
    compare_colors(&hsv_vectors, &fifth_area_colors)
}

fn compare_colors(hsv_vectors: &Vec<Vec<(u8, u8, u8)>>, fifth_area_colors: &Vec<(u8, u8, u8)>) -> Vec<Vec<String>> {
    let mut all_matched_colors = Vec::new();
    for (i, hsv_vector) in hsv_vectors.iter().enumerate() {
        let mut face_matched_colors = Vec::new();
        for (k, &(h_current, s_current, _)) in hsv_vector.iter().enumerate() {
            let mut matched_color = "".to_string();
            if k == 4 {
                matched_color = CENTER_COLORS[i].to_string();
                face_matched_colors.push(matched_color);
                continue;
            }
            
            let mut min_difference = 200;
            let mut best_match_index = 0;

            for (j, &(h_fifth, s_fifth, _)) in fifth_area_colors.iter().enumerate() {
                let h_diff = if (h_current as i16 - h_fifth as i16).abs() > 175 {
                    180 - (h_current as i16 - h_fifth as i16).abs()
                } else {
                    (h_current as i16 - h_fifth as i16).abs()
                } as u8;
                let s_diff = (s_current as i16 - s_fifth as i16).abs() as u8;

                if h_diff <= 5 && s_diff <= 128 {
                    matched_color = CENTER_COLORS[j].to_string();
                    break;
                }
                if h_diff < min_difference {
                    min_difference = h_diff;
                    best_match_index = j;
                }
            }
            if matched_color.is_empty() {
                matched_color = CENTER_COLORS[best_match_index].to_string();
            }
            face_matched_colors.push(matched_color);
        }
        all_matched_colors.push(face_matched_colors);
    }
    all_matched_colors
}