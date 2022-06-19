use ::std::ops::{Add, Div, Mul, Sub};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct VerletObject {
    pub current_position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
}

impl VerletObject {
    pub fn new(pos: Vec2, radius: f32) -> VerletObject {
        VerletObject {
            current_position: pos,
            old_position: pos,
            acceleration: Vec2::new(0.0, 0.0),
            radius: radius,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.current_position - self.old_position;
        self.old_position = self.current_position.clone();
        self.current_position = self.current_position + velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::new(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration = self.acceleration + acc;
    }
}
