extern crate image;

use std::u8;
use image::{imageops, DynamicImage, ImageBuffer, ImageResult, Luma};
use image::imageops::FilterType;
pub fn get_img(current_frame: &mut u16) -> ImageResult<DynamicImage> {
    let formatted_number = format!("output_{:04}", current_frame);
    let path_string = String::from(format!("assets\\{}.jpg", formatted_number));
    let img = image::open(&path_string);
    *current_frame += 1u16;
    img
}
pub fn grayscale_and_resize(img: DynamicImage, width: u32, height: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let gray_img = img.into_luma8();
    imageops::resize(&gray_img, width, height, FilterType::Nearest)
}
pub fn generate_ascii(resized_img: ImageBuffer<Luma<u8>, Vec<u8>>, width: u32, height: u32, chars: [&str; 6]) -> String {
    let mut ascii = String::with_capacity((width * height) as usize);
    for h in 0..height {
        for w in 0..width {
            match resized_img.get_pixel(w, h)[0] {
                200..=255 => { ascii.push(chars[0].parse().unwrap()) }
                160..200 => { ascii.push(chars[1].parse().unwrap()) }
                110..160 => { ascii.push(chars[2].parse().unwrap()) }
                75..110 => { ascii.push(chars[3].parse().unwrap()) }
                40..75 => { ascii.push(chars[4].parse().unwrap()) }
                _ => { ascii.push(chars[5].parse().unwrap()) }
            }
        }
        ascii.push("\n".parse().unwrap());
    }
    ascii
}