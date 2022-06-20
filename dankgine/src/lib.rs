#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use crate::core::engine::State;

mod core;
mod geometry;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}
