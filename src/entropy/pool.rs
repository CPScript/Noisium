use sha3::{Sha3_512, Digest};
use anyhow::{Result, anyhow};
use zeroize::Zeroize;

pub struct EntropyPool {
    pool: Vec<u8>,
    capacity: usize,
    write_pos: usize,
    extract_counter: u64,
    total_bits_added: u64,
}

impl EntropyPool {
    pub fn new(capacity_bytes: usize) -> Self {
        Self {
            pool: vec![0u8; capacity_bytes],
            capacity: capacity_bytes,
            write_pos: 0,
            extract_counter: 0,
            total_bits_added: 0,
        }
    }
    
    pub fn add_entropy(&mut self, bits: &[u8]) {
        let bytes = crate::utils::bits_to_bytes(bits);
        
        for &byte in &bytes {
            self.pool[self.write_pos] ^= byte;
            self.write_pos = (self.write_pos + 1) % self.capacity;
        }
        
        self.total_bits_added += bits.len() as u64;
    }
    
    pub fn extract(&self, num_bytes: usize) -> Result<Vec<u8>> {
        if num_bytes > 64 {
            return Err(anyhow!("Cannot extract more than 64 bytes per call"));
        }
        
        if self.total_bits_added < 8192 {
            return Err(anyhow!("Insufficient entropy collected. Wait for more data"));
        }
        
        let mut hasher = Sha3_512::new();
        hasher.update(&self.pool);
        hasher.update(&self.extract_counter.to_le_bytes());
        hasher.update(&self.write_pos.to_le_bytes());
        
        let hash = hasher.finalize();
        Ok(hash[..num_bytes].to_vec())
    }
    
    pub fn available_entropy(&self) -> usize {
        self.pool.iter().filter(|&&b| b != 0).count()
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn total_bits_collected(&self) -> u64 {
        self.total_bits_added
    }
}

impl Drop for EntropyPool {
    fn drop(&mut self) {
        self.pool.zeroize();
    }
}