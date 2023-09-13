use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct SymbolTable<T> {
    pub symbols: HashMap<Uuid, Arc<Mutex<T>>>
}

impl<T> SymbolTable<T> {
    pub(crate) fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }

    pub(crate) fn insert(&mut self, id: Uuid, species: Arc<Mutex<T>>) {
        self.symbols.insert(id, species);
    }

    pub(crate) fn lookup(&self, id: Uuid) -> Option<Arc<Mutex<T>>> {
        self.symbols.get(&id).cloned()
    }
}