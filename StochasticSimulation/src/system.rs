use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::rngs::StdRng;
use crate::reaction::Reaction;
use crate::symbol_table::SymbolTable;
use crate::visitor::Visitor;

pub struct ChemicalSystem {
    pub(crate) reactions: Vec<Arc<Mutex<Reaction>>>,
    pub(crate) symbol_table: SymbolTable<Reaction>
}

impl ChemicalSystem {
    pub fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> Arc<Mutex<ChemicalSystem>> {
        let mut symbol_table = SymbolTable::new();

        for reaction in &reactions {
            let mut reaction_guard = reaction.lock().unwrap();

            symbol_table.insert(reaction_guard.uuid, reaction.clone())
        }

        Arc::new(Mutex::new(Self {reactions, symbol_table}))
    }

    pub fn accept(system: &Arc<Mutex<ChemicalSystem>>, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        visitor.visit_system(rng, system);
    }

    pub fn simulation(&mut self, system: Arc<Mutex<ChemicalSystem>>, endTime: f64, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        let mut start_time = 0.0;

        while start_time <= endTime {
            Self::accept(&system, visitor, rng);

            let min_delay = visitor.min_delay().unwrap_or(f64::MAX);
            start_time += min_delay;

            // Print/save/monitor the state
            let system_guard = system.lock().unwrap();
            for reaction in &system_guard.reactions {
                let reaction_guard = reaction.lock().unwrap();
                println!("Reaction {}: Quantity {}", reaction_guard.name, reaction_guard.quantity);
            }
        }
    }
}