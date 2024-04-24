use crate::vec::{Point3, Vec3};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig,
            dir
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
