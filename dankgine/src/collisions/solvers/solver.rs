use super::brute_force;
use super::sweep_and_prune;
use crate::geometry::verlet::VerletObject;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CollisionSolver {}

#[allow(dead_code)]
impl CollisionSolver {
    pub fn brute_force(bodies: &mut Vec<VerletObject>) {
        brute_force::solve(bodies);
    }

    pub fn sweep_and_prune(bodies: &mut Vec<VerletObject>) {
        sweep_and_prune::solve(bodies);
    }
}
