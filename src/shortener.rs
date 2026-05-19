use crate::{code, repository::MemoryRepository, validation};

#[derive(Debug, PartialEq, Eq)]
pub enum ShortenError {
    InvalidUrl,
}

pub struct ShortenResult {
    pub code: String,
}

pub struct ShortenerService {
    repository: MemoryRepository,
}

impl ShortenerService {
    pub fn new(repository: MemoryRepository) -> Self {
        Self { repository }
    }

    pub fn shorten(&self, url: &str) -> Result<ShortenResult, ShortenError> {
        if !validation::is_valid_url(url) {
            return Err(ShortenError::InvalidUrl);
        }

        let code = loop {
            let candidate = code::generate();
            if !self.repository.contains(&candidate) {
                break candidate;
            }
        };

        self.repository.insert(code.clone(), url.to_string());

        Ok(ShortenResult { code })
    }

    pub fn resolve(&self, code: &str) -> Option<String> {
        self.repository.get(code)
    }
}
