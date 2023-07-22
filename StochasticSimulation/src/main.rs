use std::cell::RefCell;
use rand::Rng;
use rand::prelude::StdRng;
use rand::SeedableRng;

pub trait Visitor<'a> {
    fn visit_species(&mut self, species: &'a Species);
    fn visit_reactions(&mut self, reaction: &'a Reaction);
}

pub struct SystemVisitor;

impl<'a> Visitor<'a> for SystemVisitor {
    fn visit_species(&mut self, species: &'a Species) {
        species.quantity - 1;
    }

    fn visit_reactions(&mut self, reaction: &'a Reaction) {
        for species in &reaction.reactants {
            self.visit_species(species);
        }

        for species in &reaction.products {
            self.visit_species(species);
        }
    }
}

pub struct System<'a> {
    reactions: Vec<Reaction<'a>>
}

pub struct Reaction<'a> {
    reactants: Vec<&'a Species>,
    products: Vec<&'a Species>,
    delay: f64,
    lambda: f64
}

impl<'a> Reaction<'a> {
    fn accept(&'a mut self, visitor: &mut dyn Visitor<'a>) {
        visitor.visit_reactions(self);
    }

    fn compute_delay(&mut self, rng: &mut SeedableRng) {
        let mut lambda = self.lambda;

        for species in &self.reactants {
            lambda *= species.quantity.get() as f64;
        }

        let distribution = match Exp::new(lambda) {
            Ok(dist) => dist,
            Err(_) => {
                return;
            }
        };

        self.delay = distribution.sample(rng);
    }
}

pub struct Species {
    name: String,
    quantity: RefCell<i32>
}

impl<'a> Species {
    fn accept(&'a mut self, visitor: &mut dyn Visitor<'a>) {
        visitor.visit_species(self);
    }
}

fn species_builder(name: &str, quantity: i32) -> Species {
    Species { name: name.to_string(), quantity }
}

fn simulate(system: &mut System, end_time: f32) {
    let start_time: f32 = 0.0;
    let mut visitor: SystemVisitor;

    while start_time < end_time {
        for reaction in &mut system.reactions {
            reaction.accept(&mut visitor);
        }
    }
}


fn main() {
    let a = species_builder("A", 2);
    let b = species_builder("B", 1);
    let c = species_builder("C", 1);
    let d = species_builder("D", 3);
    let h = species_builder("H", 1);


    let reactants_1 = vec![&a, &b];
    let products_1 = vec![&c, &d];
    let reactants_2 = vec![&b, &d];
    let products_2 = vec![&h, &a];

    let reaction = Reaction {
        reactants: reactants_1,
        products: products_1,
        delay: 0.0,
        lambda: f64::MAX
    };
}
