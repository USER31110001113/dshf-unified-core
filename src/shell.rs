
pub struct ShellManager {
    total_layers: usize,
}

impl ShellManager {
    pub fn new(total_layers: usize) -> Self {
        Self { total_layers }
    }

    pub fn assign_layer(&self, frequency: f64) -> usize {
        let scaling_factor = (frequency * 1337.42).abs();
        let hash_bucket = scaling_factor.floor() as usize;
        hash_bucket % self.total_layers
    }

    pub fn compute_tunnel_attenuation(&self, source_layer: usize, target_layer: usize) -> f64 {
        if source_layer == target_layer {
            1.0
        } else {
            let layer_distance = (source_layer as f64 - target_layer as f64).abs();
            (-layer_distance * 0.35).exp() 
        }
    }
}