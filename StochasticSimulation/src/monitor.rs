use std::sync::{Arc, Mutex};
use crate::plotter::plot;
use crate::reaction::{Reaction, SpeciesRole};
use crate::species::Species;

pub trait Monitor<T> {
    fn record_state(&mut self, time: f64, state: &T);
}

pub trait FilterableMonitor<T>: Monitor<T> {
    fn record_state_with_filter(&mut self, time: f64, state: &T, species_to_record: &[(&str, SpeciesRole)]);
}

pub enum SnapshotData{
    Reactions(Vec<Arc<Mutex<Reaction>>>),
    SpeciesEvents(Vec<SpeciesEvents>),
}
pub struct SpeciesEvents {
    pub species_name: String,
    pub new_quantity: i32
}

#[derive(Clone)]
pub struct SystemStateSnapshot<T> {
    pub time: f64,
    pub data: SnapshotData
}

#[derive(Clone)]
pub struct DefaultMonitor {
    pub history: Vec<SystemStateSnapshot<f64>>
}

impl DefaultMonitor {
    pub fn new() -> Self {
        DefaultMonitor {
            history: Vec::new()
        }
    }

    pub fn extract_plot_data(&self, species_to_plot: &[(&str, SpeciesRole)]) -> Option<Vec<SystemStateSnapshot<Reaction>>> {
        // Iterate over each snapshot in the history to create a new vector of snapshots,
        // but only including the reactions that involve the specified species in the specified role(s).
        let filtered_snapshots: Vec<_> = self.history.iter().map(|snapshot| {

            // For each snapshot, iterate over its reactions and retain only those that
            // involve one of the specified species in the specified role(s).
            let selected_reactions: Vec<_> = match &snapshot.data {
                SnapshotData::Reactions(reactions) => {
                    reactions.iter()
                        .filter(|reaction| {
                            let reaction_guard = reaction.lock().unwrap();

                            species_to_plot.iter().any(|(name, role)| {
                                // Check if the species is in the reactants of the reaction, but only if
                                // the role we're checking for is Reactant or Both.
                                let in_reactants = if let SpeciesRole::Reactant | SpeciesRole::Both = role {
                                    reaction_guard.reactants.iter().any(|species| {
                                        let species_guard = species.lock().unwrap();
                                        &species_guard.name == name
                                    })
                                } else {
                                    false
                                };

                                // Check if the species is in the products of the reaction, but only if
                                // the role we're checking for is Product or Both.
                                let in_products = if let SpeciesRole::Product | SpeciesRole::Both = role {
                                    reaction_guard.products.iter().any(|species| {
                                        let species_guard = species.lock().unwrap();
                                        &species_guard.name == name
                                    })
                                } else {
                                    false
                                };

                                // If the species-role pair matched either a reactant or a product in the reaction,
                                // we will include this reaction in our output.
                                in_reactants || in_products
                            })
                        }).cloned().collect()
                },
                _ => vec![]
            };

            // Create a new snapshot using the filtered list of reactions and the time from the original snapshot.
            SystemStateSnapshot {
                time: snapshot.time,
                data: SnapshotData::Reactions(selected_reactions)
            }
        })
            .collect();

        // If we did not find any reactions that match the specified species-role pairs in any snapshot,
        // return None. Otherwise, return the new list of filtered snapshots.
        if filtered_snapshots.is_empty() {
            None
        } else {
            Some(filtered_snapshots)
        }
    }

    pub fn visualize_data(&self, species_to_plot: &[(&str, SpeciesRole)]) {
        if let Some(data_to_plot) = self.extract_plot_data(species_to_plot) {
            plot(&data_to_plot, &species_to_plot);
        } else {
            println!("No data to log");
        }
    }

    pub fn merge(&mut self, other: DefaultMonitor, species_to_plot: &[(&str, SpeciesRole)]) {
        self.history.extend(other.history);
    }
}

impl Monitor<Vec<Arc<Mutex<Reaction>>>> for DefaultMonitor {
    fn record_state(&mut self, time: f64, reactions: &Vec<Arc<Mutex<Reaction>>>) {
        let reaction_data: Vec<_> = reactions.iter().map(|reaction_arc| {
                let reaction = reaction_arc.lock().unwrap();

                // Create new Species instances to hold the current state
                let reactants = reaction.reactants.iter()
                    .map(|species_arc| {
                        let species = species_arc.lock().unwrap();
                        Arc::new(Mutex::new(Species {
                            name: species.name.clone(),
                            quantity: species.quantity
                        }))
                    })
                    .collect();

                let products = reaction.products.iter()
                    .map(|species_arc| {
                        let species = species_arc.lock().unwrap();
                        Arc::new(Mutex::new(Species {
                            name: species.name.clone(),
                            quantity: species.quantity
                        }))
                    })
                    .collect();

                // Create a new Reaction instance to hold the current state
                Arc::new(Mutex::new(Reaction {
                    reactants,
                    products,
                    delay: reaction.delay,
                    lambda: reaction.lambda,
                    uuid: reaction.uuid,
                    formula: reaction.formula.clone()
                }))
            })
                .collect();


        let snapshot_data = SnapshotData::Reactions(reaction_data);
        let snapshot = SystemStateSnapshot {
            time,
            data: snapshot_data
        };
        
        self.history.push(snapshot);
    }
}

impl FilterableMonitor<Vec<Arc<Mutex<Reaction>>>> for DefaultMonitor {
    fn record_state_with_filter(&mut self, time: f64, reactions: &Vec<Arc<Mutex<Reaction>>>, species_to_record: &[(&str, SpeciesRole)]) {
        let mut events = Vec::new();

        // Get the most recent snapshot if available
        if let Some(most_recent_snapshot) = self.history.last() {
            if let SnapshotData::Reactions(recent_reaction) = most_recent_snapshot {
                for reaction_arc in reactions {
                    let reaction_guard = reaction_arc.lock().unwrap();

                    for current_species_arc in reaction_guard.reactants.iter()
                        .chain(reaction_guard.products.iter()) {
                        let current_species_guard = current_species_arc.lock().unwrap();

                        for recent_reaction_arc in recent_reaction {
                            let recent_reaction_guard = recent_reaction_arc.lock().unwrap();

                            for recent_species_arc in recent_reaction_guard.reactants.iter()
                                .chain(recent_reaction_guard.products.iter()) {
                                let recent_species_guard = recent_species_arc.lock().unwrap();

                                if recent_species_guard.name == current_species_guard.name {
                                    if recent_species_guard.quantity != current_species_guard.quantity {
                                        events.push(SpeciesEvents {
                                            species_name: current_species_guard.name.clone(),
                                            new_quantity: current_species_guard.quantity
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !events.is_empty() {
            let snapshot_data = SnapshotData::SpeciesEvents(events);
            let snapshot = SystemStateSnapshot {
                time,
                data: snapshot_data
            };

            self.history.push(snapshot)
        }
    }
}