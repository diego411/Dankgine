use crate::geometry::vector::Vec2;

#[derive(Serialize, Deserialize)]
pub struct DOVerletObjects {
    pub current_positions: Vec<Vec2>,
    pub old_positions: Vec<Vec2>,
    pub accelerations: Vec<Vec2>,
    pub radius: Vec<f32>,
}

pub trait Vervelt {
    fn update_position(&mut self, dt: f32);
    fn accelerate(&mut self, acc: Vec2);
}

impl DOVerletObjects {
    pub fn new() -> DOVerletObjects {
        DOVerletObjects {
            current_positions: Vec::<Vec2>::new(),
            old_positions: Vec::<Vec2>::new(),
            accelerations: Vec::<Vec2>::new(),
            radius: Vec::new(),
        }
    }

    pub fn push(&mut self, pos: Vec2, radius: f32) {
        self.current_positions.push(pos);
        self.old_positions.push(pos);
        self.accelerations.push(Vec2::new(0.0, 0.0));
        self.radius.push(radius);
    }

    pub fn len(self) -> usize {
        self.current_positions.len()
    }

    // pub fn update_position(&mut self, dt: f32) {
    //     let velocity = self.current_position - self.old_position;
    //     self.old_position = self.current_position.clone();
    //     self.current_position = self.current_position + velocity + self.acceleration * dt * dt;
    //     self.acceleration = Vec2::new(0.0, 0.0);
    // }

    // pub fn accelerate(&mut self, acc: Vec2) {
    //     self.acceleration = self.acceleration + acc;
    // }
}
