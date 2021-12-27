use crate::bodies::{Circle, Rectangle};
pub struct World {
    pub circles: Vec<Circle>,
    pub rectangles: Vec<Rectangle>,
    pub gravity: f32,
}

impl World {
    pub fn new(gravity: f32) -> World {
        World {
            circles: Vec::new(),
            rectangles: Vec::new(),
            gravity: gravity,
        }
    }

    pub fn push_rectangle(&mut self, rect: Rectangle) -> Option<&Rectangle> {
        self.rectangles.push(rect);
        self.rectangles.last()
    }

    pub fn push_circle(&mut self, circle: Circle) -> Option<&Circle> {
        self.circles.push(circle);
        self.circles.last()
    }
}
