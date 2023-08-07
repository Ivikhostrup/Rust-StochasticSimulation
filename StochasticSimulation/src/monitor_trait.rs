use crate::system::ChemicalSystem;

pub trait Monitor {
    fn on_state_change(&mut self, system: &ChemicalSystem);
}