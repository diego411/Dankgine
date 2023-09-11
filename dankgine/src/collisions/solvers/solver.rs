use crate::{
    collisions::broad::quadtree::QuadTree,
    geometry::{rectangle::Rectangle, verlet::VerletObject},
    utils::get_two_mut,
};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CollisionSolver {}

fn solve_two_circles(i: usize, k: usize, bodies: &mut Vec<VerletObject>) {
    let (b1, b2) = match get_two_mut(i, k, bodies) {
        Some((b1, b2)) => (b1, b2),
        None => return,
    };

    let collision_axis = b1.current_position - b2.current_position;
    let dist = collision_axis.length();
    let min_dist = b1.radius + b2.radius;

    if dist == 0.0 {
        return;
    }

    if dist < min_dist {
        let n = collision_axis / dist;
        let delta = min_dist - dist;

        b1.current_position = b1.current_position + (n * 0.5 * delta);
        b2.current_position = b2.current_position - (n * 0.5 * delta);
    }
}

#[allow(dead_code)]
pub fn quadtree_solve(bodies: &mut Vec<VerletObject>) {
    let mut quadtree = QuadTree::new(Rectangle::new(0.0, 0.0, 600.0, 600.0), 32, 1, 64);

    for (index, body) in bodies.iter_mut().enumerate() {
        quadtree.insert(&body.current_position, index);
    }

    let len = bodies.len();
    for current_index in 0..len {
        let body = bodies.get(current_index).unwrap();
        let range = Rectangle::new(
            body.current_position.x - body.radius * 2.0,
            body.current_position.y - body.radius * 2.0,
            body.radius * 4.0,
            body.radius * 4.0,
        );
        let possible_collisions = quadtree.query(&range);

        for possible_collision_index in possible_collisions {
            solve_two_circles(current_index, possible_collision_index, bodies);
        }
    }
}

#[allow(dead_code)]
pub fn brute_force_solve(bodies: &mut Vec<VerletObject>) {
    let len: usize = bodies.len();

    for i in 0..len {
        for k in 0..len {
            solve_two_circles(i, k, bodies);
        }
    }
}

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
    for i in set {
        for k in set {
            solve_two_circles(*i, *k, bodies);
        }
    }
}
