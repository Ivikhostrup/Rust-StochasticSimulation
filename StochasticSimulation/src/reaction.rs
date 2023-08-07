use std::sync::{Arc, Mutex};
use rand::prelude::StdRng;
use rand::Rng;
use rand_distr::Exp;
use crate::species::Species;
use crate::symbol_table::SymbolTable;
use crate::visitor::Visitor;

pub struct Reaction {
    pub(crate) reactants: Vec<Arc<Mutex<Species>>>,
    pub(crate) products: Vec<Arc<Mutex<Species>>>,
    pub(crate) symbol_table: SymbolTable<Species>,
    pub(crate) delay: f64,
    pub(crate) lambda: f64
}

impl Reaction {
    fn new(reactants: Vec<Arc<Mutex<Species>>>,
           products: Vec<Arc<Mutex<Species>>>,
           delay: f64,
           lambda: f64) -> Arc<Mutex<Reaction>> {

        let mut symbol_table = SymbolTable::new();

        for reactant in &reactants {
            let reactant_guard = reactant.lock().unwrap();

            symbol_table.insert(reactant_guard.name.clone(), Arc::clone(reactant));
        }

        for product in &products {
            let product_guard = product.lock().unwrap();

            symbol_table.insert(product_guard.name.clone(), Arc::clone(product));
        }

        Arc::new(Mutex::new(Reaction { reactants, products, symbol_table, delay, lambda}))
    }

    fn accept(reaction: Arc<Mutex<Reaction>>, visitor: &mut dyn Visitor) {
        visitor.visit_reactions(&reaction);
    }

    pub(crate) fn compute_delay(&mut self, rng: &mut StdRng) -> f64 {
        let mut lambda = self.lambda;

        for reactant in &self.reactants {
            let reactant_guard = reactant.lock().unwrap();
            lambda *= reactant_guard.quantity as f64;
        }

        let exp = Exp::new(lambda).unwrap();
        self.delay = rng.sample(exp);
        self.delay
    }
}