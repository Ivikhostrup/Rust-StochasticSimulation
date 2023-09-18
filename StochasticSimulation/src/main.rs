mod system;
mod reaction;
mod species;
mod visitor;
mod symbol_table;
mod monitor_trait;
mod plotter;
mod monitor;

use std::sync::{Arc, Mutex};
use std::time::Instant;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use crate::monitor::DefaultMonitor;
use crate::reaction::{Reaction, SpeciesRole};
use crate::species::{Species, species_builder};
use crate::system::ChemicalSystem;
use crate::visitor::SystemVisitor;

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

    //let start_time_instant = Instant::now();
    //let mut start_time = 0.0;

    //system.simulation(2000.0, &mut visitor, &mut rng, &mut monitor);

    let num_threads = 4;
    let num_simulations = 20;
    let monitor_results: Arc<Mutex<Vec<DefaultMonitor>>> = Arc::new(Mutex::new(Vec::new()));
    let species_to_monitor = &[("A", SpeciesRole::Reactant), ("B", SpeciesRole::Product), ("C", SpeciesRole::Product)];

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    let start = Instant::now();
    pool.install(|| {
        (0..num_simulations).into_par_iter().for_each(|_| {
            let mut local_system = system.clone();
            let mut local_rng = StdRng::from_seed(seed);
            let mut local_visitor = SystemVisitor::new();
            let mut local_monitor = DefaultMonitor::new();

            local_system.simulation(2000.0,
                                    &mut local_visitor,
                                    &mut local_rng,
                                    &mut local_monitor,
                                    &species_to_monitor);

            let mut results = monitor_results.lock().unwrap();
            results.push(local_monitor);
        })
    });

    let results = monitor_results.lock().unwrap();



    let duration = start.elapsed();

    println!("Simulations took {:?}", duration);

    monitor.visualize_data(species_to_monitor);
}
