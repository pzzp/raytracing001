use crate::hitable::{HitRecored, Hitable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec::{dot, Point3};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecored> {
        let oc = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = dot(r.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        Option::Some(HitRecored::new(p, outward_normal, t, r))
    }
}
