use crate::geometry::vector::Vec2;
use crate::geometry::verlet::VerletObject;
use crate::solver::Solver;
use crate::STATE;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    bodies: Vec<VerletObject>,
}

impl State {
    pub fn new() -> State {
        State { bodies: Vec::new() }
    }
}

#[wasm_bindgen]
pub fn update() -> String {
    let solver = Solver::new();
    solver.update(0.016, &mut STATE.lock().unwrap().bodies);
    serde_json::to_string(&*STATE.lock().unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn add_body(x: f32, y: f32, radius: f32) {
    STATE
        .lock()
        .unwrap()
        .bodies
        .push(VerletObject::new(Vec2::new(x, y), radius));
}
