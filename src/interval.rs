use std::f64::INFINITY;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min: min, max: max }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
