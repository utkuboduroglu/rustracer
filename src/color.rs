use image;
use nalgebra::{Vector3};

pub type color3 = Vector3<f32>;

pub fn write_color(pixel_color: color3) -> image::Rgb<u8> {
    image::Rgb([
        (255.999 * pixel_color[0]) as u8,
        (255.999 * pixel_color[1]) as u8,
        (255.999 * pixel_color[2]) as u8,
    ])
}
