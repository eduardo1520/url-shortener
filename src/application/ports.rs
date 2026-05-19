pub trait LinkRepository: Send + Sync {
    fn get(&self, code: &str) -> Option<String>;
    fn contains(&self, code: &str) -> bool;
    fn insert(&self, code: String, url: String);
}
