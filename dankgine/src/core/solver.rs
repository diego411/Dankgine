use crate::geometry::vector::Vec2;
use crate::geometry::verlet::DOVerletObjects;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Solver {
    gravity: Vec2,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            gravity: Vec2::new(0.0, 1000.0),
        }
    }

    pub fn update(self, dt: f32, bodies: &mut DOVerletObjects) {
        let sub_steps: usize = 8;
        let sub_dt = dt / sub_steps as f32;
        for _ in 0..sub_steps {
            self.apply_gravity(bodies);
            self.apply_constraint(bodies);
            self.solve_collisions(bodies);
            self.update_position(sub_dt, bodies);
        }
    }

    fn update_position(&self, dt: f32, entities: &mut DOVerletObjects) {
        for (curr_pos, (old_pos, acc)) in entities.current_positions.iter_mut().zip(
            entities
                .old_positions
                .iter_mut()
                .zip(entities.accelerations.iter_mut()),
        ) {
            let velocity = curr_pos.clone() - old_pos.clone();
            *old_pos = curr_pos.clone();
            *curr_pos = curr_pos.clone() + velocity.clone() + acc.clone() * dt * dt;
            *acc = Vec2::new(0.0, 0.0);
        }
    }

    fn apply_gravity(self, entities: &mut DOVerletObjects) {
        for acc in entities.accelerations.iter_mut() {
            *acc = acc.clone() + self.gravity;
        }
    }

    fn apply_constraint(self, entities: &mut DOVerletObjects) {
        let constraint_position = Vec2::new(300.0, 300.0);
        let radius: f32 = 300.0;

        for (curr_pos, r) in entities
            .current_positions
            .iter_mut()
            .zip(entities.radius.iter_mut())
        {
            let diff = curr_pos.clone() - constraint_position;
            let dist = diff.length();
            if dist > radius - r.clone() {
                *curr_pos = constraint_position + (diff / dist) * (radius - r.clone());
            }
        }
    }

    fn solve_collisions(self, entities: &mut DOVerletObjects) {
        let count = entities.current_positions.len();

        for i in 0..count {
            for k in 0..count {
                let (curr1, curr2) = match get_two_mut(i, k, &mut entities.current_positions) {
                    Some((b1, b2)) => (b1, b2),
                    None => continue,
                };

                let (r1, r2) = match get_two_mut(i, k, &mut entities.radius) {
                    Some((r1, r2)) => (r1, r2),
                    None => continue,
                };

                let collision_axis = curr1.clone() - curr2.clone();
                let dist = collision_axis.length();
                let min_dist = r1.clone() + r2.clone();

                if dist < min_dist {
                    let n = collision_axis / dist;
                    let delta = min_dist - dist;

                    *curr1 = curr1.clone() + (n * 0.5 * delta);
                    *curr2 = curr2.clone() - (n * 0.5 * delta);
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
