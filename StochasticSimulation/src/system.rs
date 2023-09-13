use std::sync::{Arc, Mutex};
use std::time::Instant;
use rand::rngs::StdRng;
use crate::monitor::Monitor;
use crate::reaction::Reaction;
use crate::symbol_table::SymbolTable;
use crate::visitor::Visitor;

#[derive(Clone)]
pub struct ChemicalSystem {
    pub(crate) symbol_table: SymbolTable<Reaction>
}

impl ChemicalSystem {
    pub fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> ChemicalSystem {
        let mut symbol_table = SymbolTable::new();

        for reaction in &reactions {
            let mut reaction_guard = reaction.lock().unwrap();

            symbol_table.insert(reaction_guard.uuid, reaction.clone())
        }

        Self {symbol_table}
    }

    pub fn accept(&self, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        visitor.visit_system(rng, self);
    }

    pub fn simulation(&mut self,
                      end_time: f64,
                      visitor: &mut dyn Visitor,
                      rng: &mut StdRng,
                      monitor: &mut dyn Monitor<Vec<Arc<Mutex<Reaction>>>>) {

        let mut start_time = 0.0;

        let start_time_instant = Instant::now();
        let mut start_time = 0.0;

        while start_time <= end_time {
            self.accept(visitor, rng);

            let min_delay = visitor.min_delay().unwrap_or(f64::MAX);

            start_time += min_delay;

            let reactions_vec: Vec<_> = self.symbol_table.symbols.values().cloned().collect();
            monitor.record_state(start_time, &reactions_vec);
        }

        let duration = Instant::now() - start_time_instant;
        println!("Simulation took: {:?}", duration);
    }
}