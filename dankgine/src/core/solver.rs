use crate::{
    collisions::solvers::solver::quadtree_solve, geometry::vector::Vec2,
    geometry::verlet::VerletObject,
};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Solver {
    gravity: Vec2,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            gravity: Vec2::new(0.0, 1000.0),
        }
    }

    pub fn update(self, dt: f32, bodies: &mut Vec<VerletObject>) {
        let sub_steps: usize = 8;
        let sub_dt = dt / sub_steps as f32;
        for _ in 0..sub_steps {
            self.apply_gravity(bodies);
            self.apply_constraint(bodies);
            self.solve_collisions(bodies);
            self.update_position(sub_dt, bodies);
        }
    }

    fn update_position(&self, dt: f32, bodies: &mut Vec<VerletObject>) {
        for body in bodies {
            body.update_position(dt);
        }
    }

    fn apply_gravity(self, bodies: &mut Vec<VerletObject>) {
        for body in bodies {
            body.accelerate(self.gravity);
        }
    }

    fn apply_constraint(self, bodies: &mut Vec<VerletObject>) {
        let constraint_position = Vec2::new(300.0, 300.0);
        let radius: f32 = 300.0;
        for body in bodies {
            let diff = body.current_position - constraint_position;
            let dist = diff.length();
            if dist > radius - body.radius {
                let n = diff / dist;
                body.current_position = constraint_position + n * (radius - body.radius);
            }
        }
    }

    fn solve_collisions(self, bodies: &mut Vec<VerletObject>) {
        quadtree_solve(bodies);
    }
}
