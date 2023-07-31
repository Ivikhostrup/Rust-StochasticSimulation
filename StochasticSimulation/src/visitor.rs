use std::sync::{Arc, Mutex};
use crate::reaction::Reaction;
use crate::system::ChemicalSystem;
use crate::species::Species;
use rand::{SeedableRng};
use rand::prelude::StdRng;


pub trait Visitor<'a> {
    fn visit_system(&mut self, system: &Arc<Mutex<ChemicalSystem>>);
    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>);
    fn visit_reactants(&mut self, reactants: &Arc<Mutex<Species>>) -> Result<(), &'static str>;
    fn visit_products(&mut self, products: &Arc<Mutex<Species>>);
}

pub struct SystemVisitor {
    min_delay: Option<f64>,
    reaction_with_min_delay: Option<Arc<Mutex<Reaction>>>,
    rng: StdRng
}

impl SystemVisitor {
    fn new() -> Self {
        let seed = [0; 32];
        let rng = StdRng::from_seed(seed);
        SystemVisitor {
            min_delay: None,
            reaction_with_min_delay: None,
            rng
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
        let mut reaction_guard = reaction.lock().unwrap();
        let delay = reaction_guard.compute_delay(&mut self.rng);

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
        
        // Prevents deadlock if trying to access reaction from elsewhere
        drop(reaction_guard);

        let all_reactants_sufficient = reactants.iter().all(|reactants| {
            match self.visit_reactants(reactants) {
                Ok(()) => true,
                Err(_) => false
            }
        });

        if all_reactants_sufficient {
            for product in &products {
                self.visit_products(product)
            }
        }
    }

    fn visit_reactants(&mut self, reactants: &Arc<Mutex<Species>>) -> Result<(), &'static str> {
        let mut reactants_guard = reactants.lock().unwrap();

        if reactants_guard.quantity >= 1 {
            reactants_guard.quantity -= 1;
            Ok(())
        } else {
            Err("")
        }
    }

    fn visit_products(&mut self, products: &Arc<Mutex<Species>>) {
        let mut products_guard = products.lock().unwrap();

        products_guard.quantity += 1;
    }
}