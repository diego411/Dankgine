use std::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn length(self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, factor: f32) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, demoniator: f32) -> Self::Output {
        Self {
            x: self.x / demoniator,
            y: self.y / demoniator,
        }
    }
}
