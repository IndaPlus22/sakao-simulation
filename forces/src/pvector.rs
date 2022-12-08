// use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct PVector {
    pub x: f64,
    pub y: f64
}

impl PVector {
    pub fn new(x: f64, y: f64) -> Self {
        PVector {
            x, y
        }
    }

    pub fn gen_rand() -> Self {
        PVector {
            x: 340.0 / 2.0 + rand::random::<f64>() * 340.0,
            y: 240.0 / 2.0 + rand::random::<f64>() * 240.0
        }
    }

    pub fn add(vec1: PVector, vec2: PVector) -> PVector {
        PVector::new(vec1.x + vec2.x, vec1.y + vec2.y)
    }

    pub fn sub(vec1: PVector, vec2: PVector) -> PVector {
        PVector::new(vec1.x - vec2.x, vec1.y - vec2.y)
    }

    pub fn scale(vec: PVector, scalar: f64) -> PVector {
        PVector::new(vec.x * scalar, vec.y * scalar)
    }

    pub fn div(vec: PVector, scalar: f64) -> PVector {
        PVector::new(vec.x / scalar, vec.y / scalar)
    }

    pub fn magnitude(&mut self) -> f64 {
        f64::sqrt(f64::powi(self.x, 2) + f64::powi(self.y, 2))
    }

    pub fn normalize(&mut self) -> PVector {
        Self::div(PVector::new(self.x, self.y), self.magnitude())
    }
}