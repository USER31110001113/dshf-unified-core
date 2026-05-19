
use std::collections::{HashMap, HashSet};

pub struct EntropicGovernor {
    pub energy_threshold: f64,
}

impl EntropicGovernor {
    pub fn new(energy_threshold: f64) -> Self {
        Self { energy_threshold }
    }

    pub fn evaluate_decay(&self, amplitudes: &HashMap<String, f64>) -> HashSet<String> {
        let mut designated_for_death = HashSet::new();
        for (id, &amp) in amplitudes {
            if amp < self.energy_threshold {
                designated_for_death.insert(id.clone());
            }
        }
        designated_for_death
    }
}