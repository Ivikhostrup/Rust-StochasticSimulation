mod system;
mod reaction;
mod species;
mod visitor;
mod symbol_table;
mod monitor_trait;
mod plotter;
mod monitor;

use std::sync::{Arc, Mutex};
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::monitor::DefaultMonitor;
use crate::plotter::plot;
use crate::reaction::Reaction;
use crate::species::Species;
use crate::system::ChemicalSystem;
use crate::visitor::SystemVisitor;



fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}

fn main() {
    let a = species_builder("A", 100);
    let b = species_builder("B", 0);
    let c = species_builder("C", 1);

    let reactants = vec![a.clone(), c.clone()];
    let products = vec![b.clone(), c.clone()];

    let reaction = vec![Reaction::new(reactants, products, 0.001)];
    let mut system = ChemicalSystem::new(reaction);

    let seed = [0; 32];
    let mut rng = StdRng::from_seed(seed);

    let mut visitor = SystemVisitor::new();
    let mut monitor = DefaultMonitor::new();

    system.simulation(1000.0, &mut visitor, &mut rng, &mut monitor);
}
