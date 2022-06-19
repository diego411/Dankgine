use crate::geometry::vector::Vec2;
use crate::geometry::verlet::VerletObject;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Solver {
    gravity: Vec2,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            gravity: Vec2::new(0.0, 20000.0),
        }
    }

    pub fn update(self, dt: f32, bodies: &mut Vec<VerletObject>) {
        let sub_steps: usize = 8;
        let sub_dt = dt / sub_steps as f32;
        for _ in [0..sub_steps] {
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
        let count = bodies.len();

        for i in 0..count {
            for k in 0..count {
                let (b1, b2) = match get_two_mut(i, k, bodies) {
                    Some((b1, b2)) => (b1, b2),
                    None => continue,
                };

                let collision_axis = b1.current_position - b2.current_position;
                let dist = collision_axis.length();
                let min_dist = b1.radius + b2.radius;

                if dist < min_dist {
                    let n = collision_axis / dist;
                    let delta = min_dist - dist;

                    b1.current_position = b1.current_position + (n * 0.5 * delta);
                    b2.current_position = b2.current_position - (n * 0.5 * delta);
                }
            }
        }
    }
}

fn get_two_mut<'a, T>(i: usize, k: usize, vec: &'a mut Vec<T>) -> Option<(&'a mut T, &'a mut T)> {
    let vec_length = vec.len();
    if i == k {
        return None;
    } else if i >= vec_length || k >= vec_length {
        return None;
    }

    if i < k {
        //we want i in the left half since k will be in the right
        let (left, right) = vec.split_at_mut(i + 1);
        return Some((left.last_mut().unwrap(), right.get_mut(k - i - 1).unwrap()));
    } else {
        //i > k
        //we want i in the right half since k will be in the left
        let (left, right) = vec.split_at_mut(i);
        return Some((right.first_mut().unwrap(), left.get_mut(k).unwrap()));
    }
}
