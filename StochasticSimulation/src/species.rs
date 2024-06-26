use std::sync::{Arc, Mutex};

pub fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}

pub struct Species {
    pub(crate) name: String,
    pub(crate) quantity: i32
}

impl Species {
    fn new(name: String, quantity: i32) -> Arc<Mutex<Species>> {
        Arc::new(Mutex::new(Species {name, quantity}))
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}