mod system;
mod reaction;
mod species;
mod visitor;
mod symbol_table;
mod monitor_trait;
mod plotter;
mod monitor;

use std::time::Instant;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use crate::monitor::DefaultMonitor;
use crate::reaction::{Reaction, SpeciesRole};
use crate::species::{Species, species_builder};
use crate::system::ChemicalSystem;
use crate::visitor::SystemVisitor;


fn main() {

    let pool = rayon::ThreadPoolBuilder::build();



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

    let start_time_instant = Instant::now();
    let mut start_time = 0.0;

    system.simulation(2000.0, &mut visitor, &mut rng, &mut monitor);

    let species_to_monitor = &[("A", SpeciesRole::Reactant), ("B", SpeciesRole::Product), ("C", SpeciesRole::Product)];
    monitor.visualize_data(species_to_monitor);

    let duration = Instant::now() - start_time_instant;
    println!("Simulation with plotting took: {:?}", duration);
}
