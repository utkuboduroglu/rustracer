use super::hittable::*;
use super::ray::*;

use std::mem;
use std::rc::Rc;

// impl hittable for hittable_list
pub struct hittable_list {
    objects: Vec<Rc<dyn hittable>>,
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            objects: Vec::<Rc<dyn hittable>>::new(),
        }
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Rc<dyn hittable>) {
        self.objects.push(obj);
    }
}

impl hittable for hittable_list {
    fn hit(&self, r: &ray, t_min: f32, t_max: f32, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if (*object).hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        mem::swap(rec, &mut temp_rec);

        return hit_anything;
    }
}
