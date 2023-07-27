use std::sync::{Arc, Mutex};
use rand_distr::{Exp, Distribution};
use rand::Rng;
use rand::prelude::StdRng;
use rand::SeedableRng;

pub trait Visitor<'a> {
    fn visit_system(&mut self, system: &Arc<Mutex<ChemicalSystem>>);
    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>);
    fn visit_species(&mut self, species: &Arc<Mutex<Species>>);
}

pub struct SystemVisitor;

//impl<'a> Visitor<'a> for SystemVisitor {

//}

pub struct ChemicalSystem {
    reactions: Vec<Arc<Mutex<Reaction>>>
}

impl ChemicalSystem {
    fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> Arc<Mutex<ChemicalSystem>> {
        Arc::new(Mutex::new(Self {reactions}))
    }

    fn accept(system: Arc<Mutex<Self>>, visitor: &mut dyn Visitor) {
        visitor.visit_system(&system);
    }
}

pub struct Reaction {
    reactants: Vec<Arc<Mutex<Species>>>,
    products: Vec<Arc<Mutex<Species>>>,
    delay: f64,
    lambda: f64
}

impl Reaction {
    fn new(reactants: Vec<Arc<Mutex<Species>>>,
           products: Vec<Arc<Mutex<Species>>>,
           delay: f64,
           lambda: f64 ) -> Arc<Mutex<Reaction>> {
        Arc::new(Mutex::new(Reaction { reactants, products, delay, lambda}))
    }

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_reactions(self);
    }
}

pub struct Species {
    name: String,
    quantity: i32
}

impl Species {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_species(Arc::clone(&self));
    }
}

fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}



fn main() {

}
