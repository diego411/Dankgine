use crate::{bodies::Body, world::World};

pub struct Engine {
    pub world: World,
}

impl Engine {
    pub fn update(&mut self) {
        for circle in &mut self.world.circles {
            circle.apply_force((0.0, self.world.gravity));
        }

        for rectangle in &mut self.world.rectangles {
            rectangle.apply_force((0.0, self.world.gravity));
        }
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}
