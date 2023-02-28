use nalgebra::{Vector3};

pub type Point3 = Vector3<f32>;

pub struct ray {
    pub origin: Point3,
    pub direction: Vector3<f32>,
}

impl ray {
    pub fn new(origin: Point3, direction: Vector3<f32>) -> ray {
        ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}

pub struct camera {
    // most of this data is unnecessary; given an origin and orientation, we can derive the
    // remainder of this information.
    pub aspect_ratio: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,

    pub origin: Point3,

    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub front: Vector3<f32>,
}

impl camera {
    // TODO: implement a matrix constructor
    // pub fn new(origin: Point3, orientation: Matrix3x3<f32>) -> camera;

    pub fn rayFromUV(&self, coords: (f32, f32)) -> ray {
        let (u, v) = coords;
        // given uv coordinates, we assume them to be normalized to [0,1], apply 2u - 1, 2v -1 to
        // them before multiplying by viewport vectors
        let ray_direction = self.front + (2.0 * u - 1.0) * self.right + (2.0 * v - 1.0) * self.up;
        ray::new(self.origin, ray_direction)
    }
}
