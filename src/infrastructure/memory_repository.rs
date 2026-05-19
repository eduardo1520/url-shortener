use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::application::LinkRepository;

#[derive(Clone)]
pub struct MemoryLinkRepository {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryLinkRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl LinkRepository for MemoryLinkRepository {
    fn get(&self, code: &str) -> Option<String> {
        self.store.read().unwrap().get(code).cloned()
    }

    fn contains(&self, code: &str) -> bool {
        self.store.read().unwrap().contains_key(code)
    }

    fn insert(&self, code: String, url: String) {
        self.store.write().unwrap().insert(code, url);
    }
}
