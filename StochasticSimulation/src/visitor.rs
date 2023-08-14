use std::sync::{Arc, Mutex};
use crate::reaction::Reaction;
use crate::system::ChemicalSystem;
use crate::species::Species;
use rand::{SeedableRng};
use rand::prelude::StdRng;


pub trait Visitor {
    fn min_delay(&self) -> Option<f64>;
    fn reaction_with_min_delay(&self) -> Option<Arc<Mutex<Reaction>>>;
    fn visit_system(&mut self, rng: &mut StdRng, system: &ChemicalSystem/*monitor: &mut dyn Monitor*/);
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
    pub(crate) fn new() -> Self {
        let seed = [0; 32];
        let rng = StdRng::from_seed(seed);
        SystemVisitor {
            min_delay: None,
            reaction_with_min_delay: None,
            rng
        }
    }
}

impl Visitor for SystemVisitor {
    fn min_delay(&self) -> Option<f64> {
        self.min_delay
    }

    fn reaction_with_min_delay(&self) -> Option<Arc<Mutex<Reaction>>> {
        // Arc::clone is used
        self.reaction_with_min_delay.clone()
    }

    fn visit_system(&mut self, rng: &mut StdRng, system: &ChemicalSystem) {
        let reaction_symbol_table = &system.symbol_table;

        for reaction in reaction_symbol_table.symbols.values() {
            self.visit_reactions(reaction);
        }
    }

    fn visit_reactions(&mut self, reaction: &Arc<Mutex<Reaction>>) {
        let mut reaction_guard = reaction.lock().unwrap();
        let delay = reaction_guard.compute_delay(&mut self.rng);

        match self.min_delay {
            None => {
                self.min_delay = Some(delay);
                self.reaction_with_min_delay = Some(Arc::clone(reaction));
            }
            Some(min_delay) if delay < min_delay => {
                self.min_delay = Some(delay);
                self.reaction_with_min_delay = Some(Arc::clone(reaction));
            }
            _ => {}
        }

        // Check if reaction can happen (sufficient reactants)
        let reactants_sufficient = reaction_guard.reactants.iter().all(|reactant| {
            let mut reactant_guard = reactant.lock().unwrap();

            reactant_guard.quantity >= 1
        });

        if reactants_sufficient {
            // Perform the reaction: visit reactants and products
            for reactant in &reaction_guard.reactants {
                self.visit_reactants(reactant);
            }

            for product in &reaction_guard.products {
                self.visit_products(product);
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