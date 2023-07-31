use std::sync::{Arc, Mutex};
use crate::reaction::Reaction;
use crate::visitor::Visitor;

pub struct ChemicalSystem {
    pub(crate) reactions: Vec<Arc<Mutex<Reaction>>>
}

impl ChemicalSystem {
    fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> Arc<Mutex<ChemicalSystem>> {
        Arc::new(Mutex::new(Self {reactions}))
    }

    fn accept(system: Arc<Mutex<Self>>, visitor: &mut dyn Visitor) {
        visitor.visit_system(&system);
    }
}