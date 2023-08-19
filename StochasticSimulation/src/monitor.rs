use std::sync::{Arc, Mutex};
use crate::reaction::Reaction;
use crate::species::Species;

pub trait Monitor<T> {
    fn record_state(&mut self, time: f64, state: &T);
}

pub struct SystemStateSnapshot {
    pub time: f64,
    pub reactions: Vec<Arc<Mutex<Reaction>>>
}

pub struct DefaultMonitor {
    pub history: Vec<SystemStateSnapshot>
}

impl DefaultMonitor {
    pub fn new() -> Self {
        DefaultMonitor {
            history: Vec::new()
        }
    }

    pub fn plot_graph(&self, species_to_plot: &[&str]) {
        let filtered_data: Vec<_> = self.history.iter().map(|snapshot| {
            let filtered_reactions: Vec<_> = snapshot.reactions.iter()
                .filter(|reaction| {
                    DefaultMonitor::reaction_contains_species(reaction, species_to_plot)
                })
                .cloned()
                .collect();

            SystemStateSnapshot {
                time: snapshot.time,
                reactions: filtered_reactions
            }
        })
            .collect();
    }

    fn reaction_contains_species(reaction: &Arc<Mutex<Reaction>>, species_to_plot: &[&str]) -> bool {
        let reaction_guard = reaction.lock().unwrap();

        reaction_guard.reactants.iter()
            .chain(reaction_guard.products.iter())
            .any(|species| DefaultMonitor::species_name_is_in_list(species, species_to_plot))
    }

    fn species_name_is_in_list(species: &Arc<Mutex<Species>>, species_to_plot: &[&str]) -> bool {
        let species_guard = species.lock().unwrap();
        species_to_plot.contains(&species_guard.name.as_str())
    }
}

impl Monitor<Vec<Arc<Mutex<Reaction>>>> for DefaultMonitor {
    fn record_state(&mut self, time: f64, reactions: &Vec<Arc<Mutex<Reaction>>>) {
        let snapshot = SystemStateSnapshot {
            time,
            reactions: reactions.clone()
        };

        self.history.push(snapshot);
    }
}