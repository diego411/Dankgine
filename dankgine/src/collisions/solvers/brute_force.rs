use crate::geometry::verlet::VerletObject;
use crate::utils::get_two_mut;

#[allow(dead_code)]
pub fn solve(bodies: &mut Vec<VerletObject>) {
    let count: usize = bodies.len();

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
