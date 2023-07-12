use std::collections::HashMap;
use std::ops;

pub trait CallBack {
    fn call(&self, time: f64, chemical_system: &ChemicalSystem);
}

pub struct SpeciesQuantityRecorder {
    species_name: Vec<String>,
    monitored_species: Vec<Vec<f64>>,
    time_points: Vec<f64>
}

impl CallBack for SpeciesQuantityRecorder {
    fn call(&mut self, time: f64, chemical_system: &ChemicalSystem) {
        self.time_points.push(time);

        for (i, species_name) in self.species_name.iter().enumerate() {

        }
    }
}

pub struct Monitor {
    callback: Box<dyn CallBack>
}

impl Monitor {
    pub fn new(callback: Box<dyn CallBack>) -> Self {
        Monitor { callback }
    }

    pub fn on_state_changed(&self, time: f64, chemical_system: &ChemicalSystem) {
        self.callback.call(time, chemical_system);
    }

    pub fn get_callback(&self) -> &(dyn CallBack) {
        &*self.callback
    }
}

pub struct Species {
    name: String,
    quantity: i32
}

impl Species {
    pub fn new(name: String, quantity: i32) -> Self {
        Species {name, quantity}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_quantity(&self) -> &i32 {
        &self.quantity
    }
}

impl ops::Add for Species {
    type Output = CombinedElements;

    fn add(self, rhs: Self) -> CombinedElements {
        let mut combined = CombinedElements::new();
        combined.add(self);
        combined.add(rhs);
        combined
    }
}

pub struct CombinedElements {
    combined_species: Vec<Species>
}

impl CombinedElements {
    pub fn new() -> Self {
        CombinedElements {combined_species: Vec::new()}
    }

    pub fn add(&mut self, species: Species) {
        self.combined_species.push(species)
    }

    pub fn get_combined_species(&self) -> &Vec<Species> {
        &self.combined_species
    }
}

pub struct Reaction {
    reactants: CombinedElements,
    products: CombinedElements,
    lambda: f64,
    delay: f64
}

impl Reaction {
    pub fn new(reactants: CombinedElements, products: CombinedElements, lambda: f64) -> Self {
        Reaction {
            reactants,
            products,
            lambda,
            delay: f64::MAX
        }
    }
}

pub struct ChemicalSystem {
}

impl ChemicalSystem {

}

pub struct SymbolTable<T> {
    symbol_table: HashMap<String, T>
}

impl<T> SymbolTable<T> {
    fn new() -> Self {
        SymbolTable { symbol_table: HashMap::new()}
    }

    fn add_symbol(&mut self, name: String, object: T) {
        &self.symbol_table.entry(name).or_insert(object);
    }
}

fn main() {
    let a = Species::new(String::from("A"), 2);
    let b = Species::new(String::from("B"), 3);
    let c = Species::new(String::from("C"), 1);
    let d = Species::new(String::from("D"), 3);

    let reactants = Reaction::new(a + b, c + d, 0.001);
}
