use image;
use nalgebra::Vector3;

pub type color3 = Vector3<f32>;

pub fn write_color(pixel_color: color3, scales_per_pixel: u32) -> image::Rgb<u8> {
    // TODO: the page states that the following gamma fix (gamma=2.0) would
    // make the image brighter, but it makes it darker; though fixing shadow
    // acne actually did make it brighter!?
    let scale = 1.0/(scales_per_pixel as f32);
    let corrected_color = pixel_color
        .map(|p: f32| f32::sqrt(scale * p));

    let r = corrected_color[0];
    let g = corrected_color[1];
    let b = corrected_color[2];

    image::Rgb([
        (255.999 * f32::clamp(r, 0.0, 0.999)) as u8,
        (255.999 * f32::clamp(g, 0.0, 0.999)) as u8,
        (255.999 * f32::clamp(b, 0.0, 0.999)) as u8,
    ])
}
