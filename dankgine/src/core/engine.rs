use crate::core::solver::Solver;
use crate::geometry::vector::Vec2;
use crate::geometry::verlet::DOVerletObjects;
use crate::STATE;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    entities: DOVerletObjects,
}

impl State {
    pub fn new() -> State {
        State {
            entities: DOVerletObjects::new(),
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn update() -> String {
    let solver = Solver::new();
    solver.update(0.016, &mut STATE.lock().unwrap().entities);
    serde_json::to_string(&*STATE.lock().unwrap()).unwrap()
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn add_body(x: f32, y: f32, radius: f32) {
    STATE.lock().unwrap().entities.push(Vec2::new(x, y), radius);
}
