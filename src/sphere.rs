use super::hittable::*;
use super::ray::*;

pub struct sphere {
    center: Point3,
    radius: f32,
}

impl sphere {
    pub fn new(center: Point3, radius: f32) -> sphere {
        sphere { center, radius }
    }
}

impl hittable for sphere {
    fn hit(&self, r: &ray, t_min: f32, t_max: f32, rec: &mut hit_record) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.norm().powf(2.0);
        let half_b = oc.dot(&r.direction);
        let c = oc.norm().powf(2.0) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}
