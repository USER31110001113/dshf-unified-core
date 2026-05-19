
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct SemanticEncoder {
    min_freq: f64,
    max_freq: f64,
}

impl SemanticEncoder {
    pub fn new(min_freq: f64, max_freq: f64) -> Self {
        Self { min_freq, max_freq }
    }

    pub fn encode_string_to_frequency(&self, token: &str) -> f64 {
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        let hash_val = hasher.finish();
        let normalized = (hash_val as f64) / (u64::MAX as f64);
        self.min_freq + normalized * (self.max_freq - self.min_freq)
    }
}