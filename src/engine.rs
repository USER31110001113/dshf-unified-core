use num_complex::Complex64;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct DSHFUnifiedEngine {
    pub syntax_matrix: Vec<Complex64>,
    pub semantic_matrix: Vec<Complex64>,
    pub instruct_matrix: Vec<Complex64>,
    pub size: usize,
    pub token_to_id: HashMap<String, usize>,
    pub id_to_token: HashMap<usize, String>,
    pub phase_step: f64,
}

impl DSHFUnifiedEngine {
    pub fn new(size: usize) -> Self {
        Self {
            syntax_matrix: vec![Complex64::new(0.0, 0.0); size],
            semantic_matrix: vec![Complex64::new(0.0, 0.0); size],
            instruct_matrix: vec![Complex64::new(0.0, 0.0); size],
            size,
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            phase_step: std::f64::consts::PI / 16.0,
        }
    }

    pub fn ingest_to_channel(&mut self, text: &str, channel: &str) {
        // Pre-process common physics compounds into unified tokens during ingestion
        let mut normalized_text = text.to_lowercase()
            .replace("vertical circular motion", "vertical_circular_motion")
            .replace("kinetic energy", "kinetic_energy")
            .replace("potential energy", "potential_energy")
            .replace("mechanical energy", "mechanical_energy")
            .replace("centripetal force", "centripetal_force")
            .replace("boundary conditions", "boundary_conditions");

        let tokens: Vec<String> = normalized_text.split_whitespace()
            .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();

        let matrix = match channel.to_lowercase().as_str() {
            "syntax" => &mut self.syntax_matrix,
            "instruct" => &mut self.instruct_matrix,
            _ => &mut self.semantic_matrix,
        };

        let mut current_index = self.token_to_id.len();

        for (i, token) in tokens.iter().enumerate() {
            let id = *self.token_to_id.entry(token.clone()).or_insert_with(|| {
                let old = current_index;
                current_index += 1;
                old
            });
            self.id_to_token.insert(id, token.clone());

            let phase = (i as f64) * std::f64::consts::PI / 4.0;
            let target_idx = id % self.size;
            matrix[target_idx] += Complex64::from_polar(1.0, phase);
        }
    }

    pub fn project_tri_resonance(&mut self, query_token: &str) -> Vec<(String, f64)> {
        let cleaned = query_token.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase()
            .replace(" ", "_");
        
        let target_token = if !self.token_to_id.contains_key(&cleaned) && !cleaned.is_empty() {
            if let Some(closest) = self.token_to_id.keys().min_by_key(|&k| levenshtein_distance(k, &cleaned)) {
                closest.clone()
            } else { cleaned }
        } else { cleaned };

        let target_id = if let Some(&id) = self.token_to_id.get(&target_token) {
            id % self.size
        } else {
            return vec![];
        };

        let noise_filter = [
            "the", "a", "an", "and", "or", "but", "if", "when", "of", "at", "by", 
            "for", "with", "in", "on", "to", "is", "are", "was", "were", "be", "as"
        ];

        let laser_wave = self.semantic_matrix[target_id];
        let mut results = Vec::new();

        for (token, &id) in &self.token_to_id {
            let idx = id % self.size;
            if idx == target_id { continue; } 
            if noise_filter.contains(&token.as_str()) { continue; } 

            let sem_wave = self.semantic_matrix[idx];
            let syn_wave = self.syntax_matrix[idx];
            let ins_wave = self.instruct_matrix[idx];

            let constructive_interference = sem_wave.re * laser_wave.re + sem_wave.im * laser_wave.im;

            if constructive_interference > 0.001 {
                let total_energy = constructive_interference * (1.0 + syn_wave.norm() * 0.05 + ins_wave.norm() * 0.05);
                results.push((token.clone(), total_energy));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }

    pub fn generate_native_answer(&mut self, seed_concept: &str) -> String {
        let normalized_seed = seed_concept.trim_matches(|c: char| c.is_ascii_punctuation())
            .to_lowercase()
            .replace(" ", "_");

        let mut output = String::new();
        let mut current_context = normalized_seed;
        let mut history: Vec<String> = Vec::new();
        
        let temperature: f64 = 0.65; // Lowered slightly to prioritize structural coherence
        let token_limit = 100;
        
        output.push_str(&format!("{} ", current_context.replace("_", " ").to_uppercase()));
        history.push(current_context.clone());

        let rot = Complex64::from_polar(1.0, self.phase_step);
        let mut rng = rand::thread_rng();

        for step in 0..token_limit {
            let energy_map = self.project_tri_resonance(&current_context);
            if energy_map.is_empty() { break; }

            for i in 0..self.size {
                self.syntax_matrix[i] *= rot;
                self.semantic_matrix[i] *= rot;
            }

            let current_id = match self.token_to_id.get(&current_context) {
                Some(&id) => id,
                None => break,
            };

            let mut candidates = Vec::new();

            for (token, amplitude) in &energy_map {
                let next_id = match self.token_to_id.get(token) {
                    Some(&id) => id,
                    None => continue,
                };

                let syn_energy = self.syntax_matrix[next_id % self.size].norm();
                
                let mut sequence_bonus = 1.0;
                if next_id == current_id + 1 || (next_id % self.size) == (current_id + 1) % self.size {
                    sequence_bonus = 4.0; // Strongly prioritize sequential training streams
                }

                let mut score = amplitude * sequence_bonus * (1.0 + syn_energy * 0.1);

                let occurrences = history.iter().filter(|&t| t == token).count();
                if occurrences > 0 {
                    score *= 0.01 / (occurrences as f64);
                }

                let dither = rng.gen_range(-0.01..0.01);
                score = (score + dither).max(0.0001);

                candidates.push((token.clone(), score));
            }

            if candidates.is_empty() { break; }

            let exp_values: Vec<f64> = candidates.iter()
                .map(|(_, s)| (s / temperature).exp())
                .collect();

            let total_exp: f64 = exp_values.iter().sum();
            if total_exp <= 0.0 || total_exp.is_nan() { break; }

            let mut roll = rng.gen_range(0.0..1.0);
            let mut chosen_token = candidates[0].0.clone();

            for (i, (token, _)) in candidates.iter().enumerate() {
                let prob = exp_values[i] / total_exp;
                if roll <= prob {
                    chosen_token = token.clone();
                    break;
                }
                roll -= prob;
            }

            let display_token = chosen_token.replace("_", " ");
            output.push_str(&format!("{} ", display_token));

            // Inject natural sentence structuring
            if step > 0 && step % 12 == 0 {
                output.push_str(". ");
            }

            history.push(chosen_token.clone());
            current_context = chosen_token;
        }

        output.trim().to_string()
    }

    pub fn reinforce_channel_token(&mut self, channel: &str, token: &str, is_correct: bool) -> String {
        let cleaned = token.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase();
        let matrix = match channel.to_lowercase().as_str() {
            "syntax" => &mut self.syntax_matrix,
            "instruct" => &mut self.instruct_matrix,
            _ => &mut self.semantic_matrix,
        };

        if let Some(&id) = self.token_to_id.get(&cleaned) {
            let adjustment = if is_correct { 1.25 } else { 0.75 };
            let target_idx = id % self.size;
            let current_amp = matrix[target_idx].norm();
            let new_amp = (current_amp * adjustment).min(4096.0).max(0.01);
            let phase = matrix[target_idx].arg();
            
            matrix[target_idx] = Complex64::from_polar(new_amp, phase);
            format!("Channel [{}] | Concept '{}' scaled to amplitude: {:.4}", channel.to_uppercase(), cleaned, new_amp)
        } else {
            format!("Token '{}' unknown inside active dictionaries.", cleaned)
        }
    }

    pub fn save_memory(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(filename)?;
        let writer = std::io::BufWriter::new(file);
        bincode::serialize_into(writer, &self)?;
        Ok(())
    }

    pub fn load_memory(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);
        let loaded: DSHFUnifiedEngine = bincode::deserialize_from(reader)?;
        self.syntax_matrix = loaded.syntax_matrix;
        self.semantic_matrix = loaded.semantic_matrix;
        self.instruct_matrix = loaded.instruct_matrix;
        self.token_to_id = loaded.token_to_id;
        self.id_to_token = loaded.id_to_token;
        Ok(())
    }
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    for i in 0..=a_len { matrix[i][0] = i; }
    for j in 0..=b_len { matrix[0][j] = j; }
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1).min(matrix[i][j - 1] + 1).min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[a_len][b_len]
}