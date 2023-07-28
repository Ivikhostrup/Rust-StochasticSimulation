use std::sync::{Arc, Mutex};
use rand_distr::{Exp, Distribution};
use rand::Rng;
use rand::prelude::StdRng;
use rand::SeedableRng;

pub trait Visitor<'a> {
    fn visit_system(&mut self, system: &Arc<Mutex<ChemicalSystem>>);
    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>);
    fn visit_reactants(&mut self, reactants: &Arc<Mutex<Species>>) -> Result<(), &'static str>;
    fn visit_products(&mut self, products: &Arc<Mutex<Species>>);
}

pub struct SystemVisitor {
    min_delay: Option<f64>,
    reaction_with_min_delay: Option<Arc<Mutex<Reaction>>>
}

impl SystemVisitor {
    fn new() -> Self {
        SystemVisitor {
            min_delay: None,
            reaction_with_min_delay: None
        }
    }

    fn min_delay(&self) -> Option<f64> {
        self.min_delay
    }

    fn reaction_with_min_delay(&self) -> Option<Arc<Mutex<Reaction>>> {
        // Arc::clone is used
        self.reaction_with_min_delay.clone()
    }
}

impl<'a> Visitor<'a> for SystemVisitor {
    fn visit_system(&mut self, system: &Arc<Mutex<ChemicalSystem>>) {
        let system_guard = system.lock().unwrap();

        for reaction in &system_guard.reactions {
            self.visit_reactions(reaction);
        }
    }

    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>) {
        let reaction_guard = reaction.lock().unwrap();
        // Should be reaction.compute_delay or something
        let delay = 1.0;

        // Creating a new pointer for the Arc
        let reactants = reaction_guard.reactants.clone();
        let products = reaction_guard.products.clone();

        match self.min_delay {
            None => {
                self.min_delay = Some(delay);
                self.reaction_with_min_delay = Some(Arc::clone(reaction))
            }
            Some(min_delay) => {
                if delay < min_delay {
                    self.min_delay = Some(delay);
                    self.reaction_with_min_delay = Some(Arc::clone(reaction))
                }
            }
        }

        drop(reaction_guard);

        let all_reactants_sufficients = reactants.iter().all(|reactants| {
           match self.visit_reactants(reactants) {
               Ok(()) => true,
               Err(_) => false
           }
        });

        if all_reactants_sufficients {
            for product in &products {
                self.visit_products(product)
            }
        }
    }

    fn visit_reactants(&mut self, reactants: &Arc<Mutex<Species>>) -> Result<(), &'static str> {
        let reactants_guard = reactants.lock().unwrap();

        if reactants_guard.quantity >= 1 {
            reactants_guard.quantity -= 1;
            Ok(())
        } else {
            Err("")
        }
    }

    fn visit_products(&mut self, products: &Arc<Mutex<Species>>) {
        let products_guard = products.lock().unwrap();

        products_guard.quantity += 1;
    }
}

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
        visitor.visit_reactants(&species);
    }
}

fn species_builder(name: &str, quantity: i32) -> Arc<Mutex<Species>> {
    Arc::new(Mutex::new(Species { name: name.to_string(), quantity }))
}



fn main() {

}
