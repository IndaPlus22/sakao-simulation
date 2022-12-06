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

    pub fn add(vec1: PVector, vec2: PVector) -> PVector {
        PVector::new(vec1.x + vec2.x, vec1.y + vec2.y)
    }
}