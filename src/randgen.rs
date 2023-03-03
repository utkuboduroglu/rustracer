use nalgebra::Vector3;
use rand::{rngs::ThreadRng, Rng};

type Vector3f = Vector3<f32>;

pub fn rand_sphere_vector(rng: &mut ThreadRng) -> Vector3f {
    // generating vectors through rejection
    // embed S^2 into [-1, 1]^3
    let parametrize = |x: f32| 2.0 * x - 1.0;

    loop {
        let x = parametrize((*rng).gen::<f32>());
        let y = parametrize((*rng).gen::<f32>());
        let z = parametrize((*rng).gen::<f32>());

        if (x * x + y * y + z * z) <= 1.0 {
            return Vector3f::new(x, y, z);
        }
    }
}
