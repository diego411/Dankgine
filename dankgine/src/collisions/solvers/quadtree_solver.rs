use crate::{geometry::{verlet::VerletObject, rectangle::Rectangle}, collisions::broad::quadtree::QuadTree};

#[allow(dead_code)]
pub fn solve(bodies: &mut Vec<VerletObject>) {
    let mut quadtree = QuadTree::new(Rectangle::new(0.0, 0.0, 600.0, 600.0), 8, 1, 32);

    for body in &mut *bodies {
        quadtree.insert(*body);
    }

    for body in bodies {
        let b: VerletObject = *body;
        let range = Rectangle::new(b.current_position.x - b.radius, b.current_position.y - b.radius, b.radius * 2.0, b.radius * 2.0);
        let possible_collisions = quadtree.query(&range);
        
        if possible_collisions.is_empty() {
            continue;
        }
            
        for mut possible_collision in possible_collisions {
            if b == possible_collision {
                continue;
            }

            let collision_axis = body.current_position - possible_collision.current_position;
            let dist = collision_axis.length();
            let min_dist = body.radius + possible_collision.radius;

            if dist == 0.0 {
                continue;
            }

            if dist < min_dist {
                let n = collision_axis / dist;
                let delta = min_dist - dist;

                body.current_position = body.current_position + (n * 0.5 * delta);
                possible_collision.current_position = possible_collision.current_position - (n * 0.5 * delta);
            }
        }
        
    }    
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vec2;

    use super::*;

    #[test]
    fn solve_one_object() {
        let obj = VerletObject::new(Vec2::new(300.0, 300.0), 5.0);
        let mut bodies = Vec::new();

        bodies.push(obj);

        solve(&mut bodies);

        print!("{:?}", obj);
    }

    #[test]
    fn solve_two_colliding_objects() {
        let obj1 = VerletObject::new(Vec2::new(300.0, 300.0), 10.0);
        let obj2 = VerletObject::new(Vec2::new(295.0, 300.0), 10.0);
        let mut bodies = Vec::new();

        bodies.push(obj1);
        bodies.push(obj2);

        solve(&mut bodies);
    }
}