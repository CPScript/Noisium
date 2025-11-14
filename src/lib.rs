pub mod entropy;
pub mod crypto;
pub mod utils;

use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use zeroize::Zeroize;

pub use entropy::{EntropySource, EntropyPool, HealthMonitor, HealthStatus};
pub use crypto::{QuantumCipher, CipherAlgorithm, QuantumSigner, KeyDerivation};

pub struct QuantumCryptoEngine {
    entropy_pool: Arc<RwLock<EntropyPool>>,
    health_monitor: Arc<RwLock<HealthMonitor>>,
    collector_handle: Option<JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl QuantumCryptoEngine {
    pub fn new(pool_size_bytes: usize) -> Result<Self> {
        if pool_size_bytes < 1024 {
            anyhow::bail!("Entropy pool must be at least 1024 bytes");
        }

        let entropy_pool = Arc::new(RwLock::new(EntropyPool::new(pool_size_bytes)));
        let health_monitor = Arc::new(RwLock::new(HealthMonitor::new()));

        Ok(Self {
            entropy_pool,
            health_monitor,
            collector_handle: None,
            running: Arc::new(RwLock::new(false)),
        })
    }

    pub fn start_continuous_collection(&mut self, source: EntropySource) -> Result<()> {
        let mut running = self.running.write().unwrap();
        if *running {
            anyhow::bail!("Entropy collection already running");
        }
        *running = true;
        drop(running);

        let pool = self.entropy_pool.clone();
        let monitor = self.health_monitor.clone();
        let running_flag = self.running.clone();

        let handle = thread::spawn(move || {
            println!("Starting quantum entropy collection...");
            
            let mut consecutive_failures = 0;
            const MAX_FAILURES: u32 = 10;

            while *running_flag.read().unwrap() {
                match Self::collect_entropy_batch(&source, 2048) {
                    Ok(bits) => {
                        consecutive_failures = 0;

                        let mut monitor_guard = monitor.write().unwrap();
                        monitor_guard.update_statistics(&bits);
                        let is_healthy = monitor_guard.is_healthy();
                        drop(monitor_guard);

                        if is_healthy {
                            let mut pool_guard = pool.write().unwrap();
                            pool_guard.add_entropy(&bits);
                        } else {
                            eprintln!("WARNING: Entropy source quality degraded - skipping batch");
                        }
                    }
                    Err(e) => {
                        consecutive_failures += 1;
                        eprintln!("Entropy collection error (attempt {}/{}): {}", 
                            consecutive_failures, MAX_FAILURES, e);
                        
                        if consecutive_failures >= MAX_FAILURES {
                            eprintln!("CRITICAL: Too many consecutive failures, stopping collection");
                            *running_flag.write().unwrap() = false;
                            break;
                        }
                        
                        thread::sleep(Duration::from_secs(2));
                    }
                }

                thread::sleep(Duration::from_millis(100));
            }

            println!("Entropy collection stopped");
        });

        self.collector_handle = Some(handle);
        Ok(())
    }

    pub fn stop_collection(&mut self) {
        *self.running.write().unwrap() = false;
        
        if let Some(handle) = self.collector_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn extract_key_material(&self, num_bytes: usize) -> Result<Vec<u8>> {
        let pool = self.entropy_pool.read().unwrap();
        pool.extract(num_bytes)
    }

    pub fn health_status(&self) -> HealthStatus {
        let monitor = self.health_monitor.read().unwrap();
        monitor.current_status()
    }

    pub fn entropy_stats(&self) -> EntropyStats {
        let pool = self.entropy_pool.read().unwrap();
        let monitor = self.health_monitor.read().unwrap();
        
        EntropyStats {
            available_bytes: pool.available_entropy(),
            pool_capacity: pool.capacity(),
            health_status: monitor.current_status(),
            average_entropy: monitor.average_entropy(),
        }
    }

    fn collect_entropy_batch(source: &EntropySource, num_bits: usize) -> Result<Vec<u8>> {
        match source {
            EntropySource::Webcam => entropy::webcam::webcam_qrng(num_bits),
            EntropySource::Audio => entropy::audio::audio_qrng(num_bits),
            EntropySource::Hybrid => {
                let bits1 = entropy::webcam::webcam_qrng(num_bits)?;
                let bits2 = entropy::audio::audio_qrng(num_bits)?;
                Ok(bits1.iter().zip(bits2.iter()).map(|(a, b)| a ^ b).collect())
            }
        }
    }
}

impl Drop for QuantumCryptoEngine {
    fn drop(&mut self) {
        self.stop_collection();
    }
}

#[derive(Debug, Clone)]
pub struct EntropyStats {
    pub available_bytes: usize,
    pub pool_capacity: usize,
    pub health_status: HealthStatus,
    pub average_entropy: f64,
}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct SecureBytes(Vec<u8>);

impl SecureBytes {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}