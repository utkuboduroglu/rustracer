use nalgebra::{SMatrix, SVector, Vector3};

pub type Point3 = Vector3<f32>;
pub type Vector3f = Vector3<f32>;
pub type Matrix3f = SMatrix<f32, 3, 3>;

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
    pub origin: Point3,
    pub orientation: Matrix3f,
}

impl camera {
    // fn aspect_ratio() -> f32;
    // fn viewport_dimensions() -> (f32, f32);
    // fn focal_length() -> f32;
    // fn front() -> Vector3f;
    // fn right() -> Vector3f;
    // fn up() -> Vector3f;

    pub fn new(origin: Point3, orientation: Matrix3f) -> camera {
        camera {
            origin,
            orientation,
        }
    }

    pub fn from_vectors(origin: Point3, right: Vector3f, up: Vector3f, front: Vector3f) -> camera {
        camera {
            origin,
            orientation: Matrix3f::from_columns(&[right, up, front]),
        }
    }

    fn right(&self) -> Vector3f {
        self.orientation * Vector3f::new(1.0, 0.0, 0.0)
    }

    fn up(&self) -> Vector3f {
        self.orientation * Vector3f::new(0.0, 1.0, 0.0)
    }

    // should this really be -1? Maybe leave orientation to the user
    fn front(&self) -> Vector3f {
        self.orientation * Vector3f::new(0.0, 0.0, 1.0)
    }

    fn viewport_dimensions(&self) -> (f32, f32) {
        // the relevant vectors are right and up
        // we do a simple math calculation for the norms
        let vec_norm = |vec: &Vector3f| f32::sqrt((*vec).dot(vec));

        let viewport_width = vec_norm(&(self.right()));
        let viewport_height = vec_norm(&(self.up()));

        (viewport_width, viewport_height)
    }

    fn aspect_ratio(&self) -> f32 {
        // the aspect ratio is practically just viewport_width/viewport_height
        let (w, h) = self.viewport_dimensions();
        w / h
    }

    fn focal_length(&self) -> f32 {
        let vec_norm = |vec: &Vector3f| f32::sqrt((*vec).dot(vec));
        f32::sqrt(vec_norm(&(self.front())))
    }

    pub fn rayFromUV(&self, coords: (f32, f32)) -> ray {
        let (u, v) = coords;
        // given uv coordinates, we assume them to be normalized to [0,1], apply 2u - 1, 2v -1 to
        // them before multiplying by viewport vectors
        let ray_direction =
            self.front() + (2.0 * u - 1.0) * self.right() + (2.0 * v - 1.0) * self.up();
        ray::new(self.origin, ray_direction)
    }
}
