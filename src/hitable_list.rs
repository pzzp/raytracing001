use std::rc::Rc;

use crate::{
    hitable::{HitRecored, Hitable},
    interval::Interval,
    ray::Ray,
};

pub struct HitableList {
    objects: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new_empty() -> Self {
        HitableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hitable>) {
        self.objects.push(object);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecored> {
        let mut result = Option::<HitRecored>::None;
        let mut closet_so_far = ray_t.max;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, &Interval::new(ray_t.min, closet_so_far)) {
                result = Some(rec);
                closet_so_far = rec.t;
            }
        }
        result
    }
}
