use std::sync::{Arc, Mutex};
use crate::reaction::Reaction;

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