use crate::ray::Ray;
use crate::vec::{dot, Point3};
use crate::hitable::{HitRecored, Hitable};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hitable for Sphere{
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecored> {
        let oc = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = dot(r.dir, oc);
        let c =  oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        Option::Some(HitRecored::new(p, outward_normal, t, r))
    }
}