use crate::interval::{self, Interval};
use crate::ray::Ray;
use crate::vec::{dot, Point3, Vec3};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct HitRecored {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecored {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, r: &Ray) -> Self {
        let front_face = dot(r.dir, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecored>;
}
