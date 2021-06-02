use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Float2 {
        Float2 { x, y }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn set_as(&mut self, other: &Float2) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn set_length(&mut self, scale: f32) {
        self.normalize();
        
        self.x *= scale;
        self.y *= scale;
    }

    pub fn normalize(&mut self) {
        let size = self.size();

        self.x /= size;
        self.y /= size;
    }

    pub fn size(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn rotation(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl Sub for Float2 {
    type Output = Float2;

    fn sub(self, other: Float2) -> Self::Output {
        Float2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Float2 {
    type Output = Float2;

    fn add(self, other: Float2) -> Self::Output {
        Float2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Default for Float2 {
    fn default() -> Self {
        Float2::new(0.0, 0.0)
    }
}

impl Display for Float2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Float2 x: {} y: {}", self.x, self.y)
    }
}
