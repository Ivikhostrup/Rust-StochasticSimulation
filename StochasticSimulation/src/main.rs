mod system;
mod reaction;
mod species;
mod visitor;
mod symbol_table;
mod monitor_trait;
use std::sync::{Arc, Mutex};
use crate::species::Species;

fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}

fn main() {

}
