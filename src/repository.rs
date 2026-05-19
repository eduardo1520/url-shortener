use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct MemoryRepository {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, code: &str) -> Option<String> {
        self.store.read().unwrap().get(code).cloned()
    }

    pub fn contains(&self, code: &str) -> bool {
        self.store.read().unwrap().contains_key(code)
    }

    pub fn insert(&self, code: String, url: String) {
        self.store.write().unwrap().insert(code, url);
    }
}
