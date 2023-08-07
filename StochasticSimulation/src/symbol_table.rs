use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SymbolTable<T> {
    symbols: HashMap<String, Arc<Mutex<T>>>
}

impl<T> SymbolTable<T> {
    pub(crate) fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }

    pub(crate) fn insert(&mut self, name: String, species: Arc<Mutex<T>>) {
        self.symbols.insert(name, species);
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<Arc<Mutex<T>>> {
        self.symbols.get(name).cloned()
    }
}