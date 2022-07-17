use crate::geometry::vector::Vec2;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct VerletObject {
    pub current_position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
}

pub trait Vervelt {
    fn update_position(&mut self, dt: f32);
    fn accelerate(&mut self, acc: Vec2);
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
