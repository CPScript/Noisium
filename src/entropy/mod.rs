pub mod webcam;
pub mod audio;
pub mod pool;
pub mod health;

pub use pool::EntropyPool;
pub use health::{HealthMonitor, HealthStatus};

#[derive(Debug, Clone)]
pub enum EntropySource {
    Webcam,
    Audio,
    Hybrid,
}