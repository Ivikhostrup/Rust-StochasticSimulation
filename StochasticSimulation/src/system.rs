use std::sync::{Arc, Mutex};
use rand::rngs::StdRng;
use crate::reaction::Reaction;
use crate::visitor::Visitor;

pub struct ChemicalSystem {
    pub(crate) reactions: Vec<Arc<Mutex<Reaction>>>
}

impl ChemicalSystem {
    fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> Arc<Mutex<ChemicalSystem>> {
        Arc::new(Mutex::new(Self {reactions}))
    }

    fn accept(system: &Arc<Mutex<Self>>, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        visitor.visit_system(rng, system);
    }

    fn simulation(system: &Arc<Mutex<Self>>, endTime: f64, visitor: &mut dyn Visitor, rng: &mut StdRng) {
        let mut start_time = 0.0;

        while start_time <= endTime {
            Self::accept(system, visitor, rng);

            let min_delay = visitor.min_delay().unwrap_or(f64::MAX);
            start_time += min_delay;

            // The reaction with minimum delay is handled here now
            if let Some(reaction) = visitor.reaction_with_min_delay() {
                let mut reaction_guard = reaction.lock().unwrap();

            }
        }
    }
}