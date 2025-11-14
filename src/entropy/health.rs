use std::collections::VecDeque;

pub struct HealthMonitor {
    recent_samples: VecDeque<f64>,
    window_size: usize,
    entropy_threshold: f64,
    bias_threshold: f64,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            recent_samples: VecDeque::with_capacity(100),
            window_size: 100,
            entropy_threshold: 0.85,
            bias_threshold: 0.15,
        }
    }
    
    pub fn update_statistics(&mut self, bits: &[u8]) {
        let entropy = estimate_entropy(bits);
        
        self.recent_samples.push_back(entropy);
        if self.recent_samples.len() > self.window_size {
            self.recent_samples.pop_front();
        }
    }
    
    pub fn is_healthy(&self) -> bool {
        if self.recent_samples.len() < 10 {
            return true;
        }
        
        let avg_entropy = self.average_entropy();
        avg_entropy >= self.entropy_threshold
    }
    
    pub fn average_entropy(&self) -> f64 {
        if self.recent_samples.is_empty() {
            return 0.0;
        }
        
        self.recent_samples.iter().sum::<f64>() / self.recent_samples.len() as f64
    }
    
    pub fn current_status(&self) -> HealthStatus {
        if self.recent_samples.is_empty() {
            return HealthStatus::Initializing;
        }
        
        let avg_entropy = self.average_entropy();
        
        if avg_entropy >= 0.95 {
            HealthStatus::Excellent
        } else if avg_entropy >= self.entropy_threshold {
            HealthStatus::Good
        } else if avg_entropy >= 0.70 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Failed
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    Initializing,
    Excellent,
    Good,
    Degraded,
    Failed,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Initializing => write!(f, "Initializing"),
            HealthStatus::Excellent => write!(f, "Excellent"),
            HealthStatus::Good => write!(f, "Good"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Failed => write!(f, "Failed"),
        }
    }
}

fn estimate_entropy(bits: &[u8]) -> f64 {
    if bits.is_empty() {
        return 0.0;
    }
    
    let ones = bits.iter().filter(|&&b| b == 1).count();
    let zeros = bits.len() - ones;
    
    let p0 = zeros as f64 / bits.len() as f64;
    let p1 = ones as f64 / bits.len() as f64;
    
    let mut entropy = 0.0;
    if p0 > 0.0 {
        entropy -= p0 * p0.log2();
    }
    if p1 > 0.0 {
        entropy -= p1 * p1.log2();
    }
    
    entropy
}