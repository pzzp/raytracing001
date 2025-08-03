use std::rc::Rc;

use crate::hitable::{HitRecored, Hitable};

pub struct HitableList {
    objects: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new_empty() -> Self {
        HitableList {
            objects: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hitable>) {
        self.objects.push(object);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecored> {
        let mut result = Option::<HitRecored>::None;
        let mut closet_so_far = ray_tmax;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, ray_tmin, closet_so_far) {
                result = Some(rec);
                closet_so_far = rec.t;
            }
        }
        result
    }
}