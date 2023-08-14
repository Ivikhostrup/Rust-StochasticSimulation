use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::rngs::StdRng;
use crate::reaction::Reaction;
use crate::symbol_table::SymbolTable;
use crate::visitor::Visitor;

pub struct ChemicalSystem {
    //pub(crate) reactions: Vec<Arc<Mutex<Reaction>>>,
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

    pub fn simulation(&mut self, endTime: f64, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        let mut start_time = 0.0;

        while start_time <= endTime {
            self.accept(visitor, rng);

            let min_delay = visitor.min_delay().unwrap_or(f64::MAX);
            start_time += min_delay;

            // Print/save/monitor the state
            for reaction in self.symbol_table.symbols.values() {
                let reaction_guard = reaction.lock().unwrap();
                reaction_guard.print_details();
            }
        }
    }
}