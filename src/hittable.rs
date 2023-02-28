use super::ray::*;
use nalgebra::Vector3;

pub struct hit_record {
    pub p: Point3,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
}

impl hit_record {
    pub fn new() -> hit_record {
        hit_record {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &ray, outward_normal: &Vector3<f32>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait hittable {
    fn hit(&self, r: &ray, t_min: f32, t_max: f32, rec: &mut hit_record) -> bool;
}
