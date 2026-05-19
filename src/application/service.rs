use crate::domain::{code, validation};

use super::{LinkRepository, ShortenError};

pub struct ShortenResult {
    pub code: String,
}

pub struct ShortenerService<R: LinkRepository> {
    repo: R,
}

impl<R: LinkRepository> ShortenerService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn shorten(&self, url: &str) -> Result<ShortenResult, ShortenError> {
        if !validation::is_valid_url(url) {
            return Err(ShortenError::InvalidUrl);
        }

        let code = loop {
            let candidate = code::generate();
            if !self.repo.contains(&candidate) {
                break candidate;
            }
        };

        self.repo.insert(code.clone(), url.to_string());

        Ok(ShortenResult { code })
    }

    pub fn resolve(&self, code: &str) -> Option<String> {
        self.repo.get(code)
    }
}
