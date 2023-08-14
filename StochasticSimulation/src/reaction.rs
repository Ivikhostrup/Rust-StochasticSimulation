use std::sync::{Arc, Mutex};
use rand::prelude::StdRng;
use rand::Rng;
use rand_distr::Exp;
use crate::species::Species;
use crate::symbol_table::SymbolTable;
use crate::visitor::Visitor;
use uuid::Uuid;

pub struct Reaction {
    pub(crate) reactants: Vec<Arc<Mutex<Species>>>,
    pub(crate) products: Vec<Arc<Mutex<Species>>>,
    pub(crate) delay: f64,
    pub(crate) lambda: f64,
    pub(crate) uuid: Uuid,
    pub(crate) formula: String
}

impl Reaction {
    //noinspection ALL
    pub fn new(reactants: Vec<Arc<Mutex<Species>>>,
           products: Vec<Arc<Mutex<Species>>>,
           lambda: f64) -> Arc<Mutex<Reaction>> {

        let uuid = Uuid::new_v4();
        let delay = f64::MAX;

        let reactant_str = reactants.iter()
            .map(|reactant| {
                let reactant_guard = reactant.lock().unwrap();
                reactant_guard.name.clone()
            })
            // turbofish - You specify what type should come out of collect
            .collect::<Vec<String>>()
            .join(" + ");

        let product_str = products.iter()
            .map(|product| {
                let product_guard = product.lock().unwrap();
                product_guard.name.clone()
            })
            .collect::<Vec<String>>()
            .join(" + ");

        let formula = format!("{} -> {}", reactant_str, product_str);

        Arc::new(Mutex::new(Reaction { reactants, products, delay, lambda, uuid, formula}))
    }

    fn accept(reaction: &Arc<Mutex<Reaction>>, visitor: &mut dyn Visitor) {
        visitor.visit_reactions(reaction);
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

    pub fn print_details(&self) {
        //println!("Formula {}", self.formula);

        print!("Reactants: ");
        // Enumerate provides tuple with index and value
        // Used to keep track of position
        for (index, reactant) in self.reactants.iter().enumerate() {
            let reactant_guard = reactant.lock().unwrap();
            print!("{} {}", reactant_guard.name, reactant_guard.quantity);

            if index < self.reactants.len() - 1 {
                print!(", ");
            }
        }

        println!();

        print!("Products:  ");
        for (index, products) in self.products.iter().enumerate() {
            let product_guard = products.lock().unwrap();
            print!("{} {}", product_guard.name, product_guard.quantity);

            if index < self.products.len() - 1 {
                print!(", ");
            }
        }
        println!();
    }
}