use crate::QuantumCryptoEngine;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey};
use anyhow::{Result, anyhow};
use std::sync::Arc;
use zeroize::Zeroize;

#[derive(Debug, Clone, Copy)]
pub enum CipherAlgorithm {
    AES256GCM,
    ChaCha20Poly1305,
}

pub struct QuantumCipher {
    engine: Arc<QuantumCryptoEngine>,
    algorithm: CipherAlgorithm,
}

impl QuantumCipher {
    pub fn new(engine: Arc<QuantumCryptoEngine>, algorithm: CipherAlgorithm) -> Self {
        Self { engine, algorithm }
    }
    
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CipherAlgorithm::AES256GCM => self.encrypt_aes(plaintext),
            CipherAlgorithm::ChaCha20Poly1305 => self.encrypt_chacha(plaintext),
        }
    }
    
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CipherAlgorithm::AES256GCM => self.decrypt_aes(ciphertext),
            CipherAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha(ciphertext),
        }
    }
    
    fn encrypt_aes(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut key_material = self.engine.extract_key_material(32)?;
        let mut nonce_material = self.engine.extract_key_material(12)?;
        
        let cipher = Aes256Gcm::new_from_slice(&key_material)
            .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
        let nonce = Nonce::from_slice(&nonce_material);
        
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        let mut result = Vec::with_capacity(1 + 12 + ciphertext.len());
        result.push(0x01);
        result.extend_from_slice(&nonce_material);
        result.extend_from_slice(&ciphertext);
        
        key_material.zeroize();
        nonce_material.zeroize();
        
        Ok(result)
    }
    
    fn decrypt_aes(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 13 || data[0] != 0x01 {
            return Err(anyhow!("Invalid ciphertext format"));
        }
        
        let mut key_material = self.engine.extract_key_material(32)?;
        let nonce_bytes = &data[1..13];
        let ciphertext = &data[13..];
        
        let cipher = Aes256Gcm::new_from_slice(&key_material)
            .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        key_material.zeroize();
        
        Ok(plaintext)
    }
    
    fn encrypt_chacha(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut key_material = self.engine.extract_key_material(32)?;
        let mut nonce_material = self.engine.extract_key_material(12)?;
        
        let key = ChaChaKey::from_slice(&key_material);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_material);
        
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        let mut result = Vec::with_capacity(1 + 12 + ciphertext.len());
        result.push(0x02);
        result.extend_from_slice(&nonce_material);
        result.extend_from_slice(&ciphertext);
        
        key_material.zeroize();
        nonce_material.zeroize();
        
        Ok(result)
    }
    
    fn decrypt_chacha(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 13 || data[0] != 0x02 {
            return Err(anyhow!("Invalid ciphertext format"));
        }
        
        let mut key_material = self.engine.extract_key_material(32)?;
        let nonce_bytes = &data[1..13];
        let ciphertext = &data[13..];
        
        let key = ChaChaKey::from_slice(&key_material);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        key_material.zeroize();
        
        Ok(plaintext)
    }
}