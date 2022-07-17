use crate::geometry::verlet::VerletObject;
use crate::utils::get_two_mut;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve(bodies: &mut Vec<crate::geometry::verlet::VerletObject>) {
    let count = bodies.len();

    bodies.sort_by(|a, b| {
        a.current_position
            .x
            .partial_cmp(&b.current_position.x)
            .unwrap()
    });

    let mut active_intervall: Option<(f32, f32)> = None;
    let mut set: HashSet<usize> = HashSet::new();

    for i in 0..count {
        let current = bodies.get(i).unwrap();
        let current_min_x = current.current_position.x - current.radius;
        let current_max_x = current.current_position.x + current.radius;
        match active_intervall {
            Some(intervall) => {
                if current_min_x <= intervall.1 && current_max_x >= intervall.0 {
                    //current object in active intervall
                    active_intervall = Some((
                        f32::min(intervall.0, current_min_x),
                        f32::max(intervall.1, current_max_x),
                    ));
                    set.insert(i);
                } else {
                    //current object not in active intervall
                    //Narrow Phase: Are they actually colliding?
                    solve_collisions_for_set(&set, bodies);
                    set = HashSet::new();
                    set.insert(i);
                    active_intervall = Some((current_min_x, current_max_x));
                }
            }
            None => {
                active_intervall = Some((current_min_x, current_max_x));
                set.insert(i);
            }
        }
    }
}

fn solve_collisions_for_set(set: &HashSet<usize>, bodies: &mut Vec<VerletObject>) {
    for index1 in set {
        for index2 in set {
            let (b1, b2) = match get_two_mut(index1.clone(), index2.clone(), bodies) {
                Some((b1, b2)) => (b1, b2),
                None => continue,
            };
            let collision_axis = b1.current_position - b2.current_position;
            let dist = collision_axis.length();
            let min_dist = b1.radius + b2.radius;

            if dist < min_dist {
                let delta = min_dist - dist;
                let n = collision_axis / dist; //TODO: this can crash when dist == 0

                //Solve Collision
                b1.current_position = b1.current_position + (n * 0.5 * delta);
                b2.current_position = b2.current_position - (n * 0.5 * delta);
            }
        }
    }
}
