use crate::QuantumCryptoEngine;
use sha3::{Sha3_256, Sha3_512, Digest};
use anyhow::Result;
use std::sync::Arc;
use zeroize::Zeroize;

pub struct KeyDerivation {
    engine: Arc<QuantumCryptoEngine>,
}

impl KeyDerivation {
    pub fn new(engine: Arc<QuantumCryptoEngine>) -> Self {
        Self { engine }
    }
    
    pub fn derive_key(&self, purpose: &[u8], output_len: usize) -> Result<Vec<u8>> {
        if output_len > 64 {
            return self.derive_key_extended(purpose, output_len);
        }
        
        let mut entropy = self.engine.extract_key_material(32)?;
        
        let mut hasher = Sha3_512::new();
        hasher.update(&entropy);
        hasher.update(purpose);
        hasher.update(&output_len.to_le_bytes());
        
        let result = hasher.finalize();
        entropy.zeroize();
        
        Ok(result[..output_len].to_vec())
    }
    
    fn derive_key_extended(&self, purpose: &[u8], output_len: usize) -> Result<Vec<u8>> {
        let mut entropy = self.engine.extract_key_material(32)?;
        let mut output = Vec::with_capacity(output_len);
        let mut counter = 0u64;
        
        while output.len() < output_len {
            let mut hasher = Sha3_512::new();
            hasher.update(&entropy);
            hasher.update(purpose);
            hasher.update(&counter.to_le_bytes());
            
            let hash = hasher.finalize();
            let remaining = output_len - output.len();
            let to_take = remaining.min(hash.len());
            output.extend_from_slice(&hash[..to_take]);
            
            counter += 1;
        }
        
        entropy.zeroize();
        Ok(output)
    }
    
    pub fn derive_subkey(&self, master_key: &[u8], purpose: &[u8], output_len: usize) -> Result<Vec<u8>> {
        if output_len > 64 {
            return self.derive_subkey_extended(master_key, purpose, output_len);
        }
        
        let mut hasher = Sha3_512::new();
        hasher.update(master_key);
        hasher.update(purpose);
        hasher.update(&output_len.to_le_bytes());
        
        let result = hasher.finalize();
        Ok(result[..output_len].to_vec())
    }
    
    fn derive_subkey_extended(&self, master_key: &[u8], purpose: &[u8], output_len: usize) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(output_len);
        let mut counter = 0u64;
        
        while output.len() < output_len {
            let mut hasher = Sha3_512::new();
            hasher.update(master_key);
            hasher.update(purpose);
            hasher.update(&counter.to_le_bytes());
            
            let hash = hasher.finalize();
            let remaining = output_len - output.len();
            let to_take = remaining.min(hash.len());
            output.extend_from_slice(&hash[..to_take]);
            
            counter += 1;
        }
        
        Ok(output)
    }
    
    pub fn generate_password(&self, length: usize, charset: PasswordCharset) -> Result<String> {
        let chars = charset.get_charset();
        let entropy_bytes = self.engine.extract_key_material((length * 2).min(64))?;
        
        let mut hasher = Sha3_256::new();
        hasher.update(&entropy_bytes);
        let hash = hasher.finalize();
        
        let mut password = String::with_capacity(length);
        for i in 0..length {
            let idx = hash[i % hash.len()] as usize % chars.len();
            password.push(chars[idx]);
        }
        
        Ok(password)
    }
}

pub enum PasswordCharset {
    Alphanumeric,
    AlphanumericSymbols,
    Hex,
    Base64,
}

impl PasswordCharset {
    fn get_charset(&self) -> Vec<char> {
        match self {
            PasswordCharset::Alphanumeric => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                    .chars().collect()
            }
            PasswordCharset::AlphanumericSymbols => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:,.<>?"
                    .chars().collect()
            }
            PasswordCharset::Hex => {
                "0123456789abcdef".chars().collect()
            }
            PasswordCharset::Base64 => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                    .chars().collect()
            }
        }
    }
}