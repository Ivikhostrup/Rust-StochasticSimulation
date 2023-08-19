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
        // Iterate over the history and extract reactions based on the species of interest
        let filtered_snapshots: Vec<_> = self.history.iter().map(|snapshot| {

            // Filter reactions in the snapshot based on whether they contain any species of interest
            let selected_reactions: Vec<_> = snapshot.reactions.iter()
                .filter(|reaction| {
                    let reaction_guard = reaction.lock().unwrap();

                    // Check both reactants and products for the species of interest
                    reaction_guard.reactants.iter()
                        .chain(reaction_guard.products.iter())
                        .any(|species| {
                            let species_guard = species.lock().unwrap();

                            // Check if the current species is in the list of species to plot
                            species_to_plot.contains(&species_guard.name.as_str())
                        })
                })
                .cloned()  // Clone the Arc<Mutex<Reaction>> to create the filtered list
                .collect();

            // Create a new snapshot with only the filtered reactions
            SystemStateSnapshot {
                time: snapshot.time,
                reactions: selected_reactions
            }
        })
            .collect();
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