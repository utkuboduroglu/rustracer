use image;
use nalgebra::Vector3;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::rc::Rc;

// do this without explicitly creating modules?
mod color;
mod hittable;
mod hittable_list;
mod randgen;
mod rasterizable;
mod ray;
mod sphere;

use color::*;
use hittable::*;
use randgen::*;
use rasterizable::*;
use ray::*;

const output_filename: &str = "output_image.png";
const pixel_samples: u32 = 8;
const max_depth: u32 = 24;

fn ray_color(r: &ray::ray, world: &dyn hittable::hittable, depth: u32) -> color3 {
    // are we constantly going to create an rng generator for each ray color?
    let mut rng: ThreadRng = rand::thread_rng();

    if depth == 0 {
        return color3::new(0.0, 0.0, 0.0);
    }

    let mut rec = hit_record::new();
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.normal + rand_sphere_vector(&mut rng).normalize();
        return 0.5 * ray_color(&ray::ray::new(rec.p, target), world, depth - 1);
        // return 0.5 * (rec.normal + color3::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * color3::new(1.0, 1.0, 1.0) + t * color3::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let aspect_ratio = 640.0 / 480.0;
    let pixelResolution: u32 = 480;

    let canvas = canvas::new(pixelResolution, aspect_ratio, 1.0);
    // should canvases contain their own image buffers?

    let (dimX, dimY) = canvas.dimensionsF32();
    println!("{:?}", canvas.dimensions());

    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(dimX as u32, dimY as u32);

    let mut world = hittable_list::hittable_list::new();
    world.add(Rc::new(sphere::sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(sphere::sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let viewport_height = 1.0;
    let viewport_width = aspect_ratio * viewport_height;

    let camera = camera::from_vectors(
        Vector3f::new(0.0, 0.0, 0.0),
        Vector3f::new(viewport_width, 0.0, 0.0),
        Vector3f::new(0.0, -viewport_height, 0.0), // this needs to be negative to work?
        Vector3f::new(0.0, 0.0, -1.0),
    );

    // the core render loop: can this be made more rust-like?
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut scanline = dimY as u32 - j;
        if i == 0 {
            scanline = dimY as u32 - j;
        }

        let mut pixel_color = color3::new(0.0, 0.0, 0.0);
        for s in 0..pixel_samples {
            eprint!("\r[scanline: {}] Pixel sample: {}", scanline, s);
            // BUG: the current render is completely black
            let uv_coords = (
                (rng.gen::<f32>() + i as f32) / (dimX - 1.0),
                (rng.gen::<f32>() + j as f32) / (dimY - 1.0),
            );
            // the most memory use is probably due to these calls
            let pixel_ray = camera.rayFromUV(uv_coords);
            pixel_color += ray_color(&pixel_ray, &mut world, max_depth);
        }

        *pixel = write_color(pixel_color, pixel_samples);
    }

    imgbuf.save(output_filename).unwrap();
}
