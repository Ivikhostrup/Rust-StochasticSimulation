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

impl<'a> Visitor<'a> for SystemVisitor {
    fn visit_system(&mut self, system: &Arc<Mutex<ChemicalSystem>>) {
        let system = system.lock().unwrap();

        for reaction in &system.reactions {
            self.visit_reactions(reaction);
        }
    }

    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>) {
        let reaction = reaction.lock().unwrap();
        let min_delay = 1;

        
    }

    fn visit_species(&mut self, species: &Arc<Mutex<Species>>) {
        todo!()
    }
}

pub struct ChemicalSystem {
    reactions: Vec<Arc<Mutex<Reaction>>>,
    min_delay: Option<f64>,
    reaction_with_min_delay: Option<Arc<Mutex<f64>>>
}

impl ChemicalSystem {
    fn new(reactions: Vec<Arc<Mutex<Reaction>>>) -> Arc<Mutex<ChemicalSystem>> {
        Arc::new(Mutex::new(Self {reactions, min_delay: None}))
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

    fn accept(reaction: Arc<Mutex<Reaction>>, visitor: &mut dyn Visitor) {
        visitor.visit_reactions(&reaction);
    }

    fn compute_delay() {

    }
}

pub struct Species {
    name: String,
    quantity: i32
}

impl Species {
    fn new(name: String, quantity: i32) -> Arc<Mutex<Species>> {
        Arc::new(Mutex::new(Species {name, quantity}))
    }

    fn accept(species: Arc<Mutex<Species>>, visitor: &mut dyn Visitor) {
        visitor.visit_species(&species);
    }
}

fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}



fn main() {

}
