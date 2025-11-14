use crate::QuantumCryptoEngine;
use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use anyhow::{Result, anyhow};
use std::sync::Arc;
use zeroize::Zeroize;

pub struct QuantumSigner {
    engine: Arc<QuantumCryptoEngine>,
}

impl QuantumSigner {
    pub fn new(engine: Arc<QuantumCryptoEngine>) -> Self {
        Self { engine }
    }
    
    pub fn generate_keypair(&self) -> Result<(SigningKey, VerifyingKey)> {
        let mut seed = self.engine.extract_key_material(32)?;
        
        let seed_array: [u8; 32] = seed[..32].try_into()
            .map_err(|_| anyhow!("Invalid key material length"))?;
        
        let signing_key = SigningKey::from_bytes(&seed_array);
        let verifying_key = signing_key.verifying_key();
        
        seed.zeroize();
        
        Ok((signing_key, verifying_key))
    }
    
    pub fn sign(&self, message: &[u8], signing_key: &SigningKey) -> Signature {
        signing_key.sign(message)
    }
    
    pub fn verify(&self, message: &[u8], signature: &Signature, verifying_key: &VerifyingKey) -> Result<()> {
        verifying_key.verify(message, signature)
            .map_err(|e| anyhow!("Signature verification failed: {}", e))
    }
    
    pub fn export_signing_key(&self, key: &SigningKey) -> Vec<u8> {
        key.to_bytes().to_vec()
    }
    
    pub fn export_verifying_key(&self, key: &VerifyingKey) -> Vec<u8> {
        key.to_bytes().to_vec()
    }
    
    pub fn import_signing_key(&self, bytes: &[u8]) -> Result<SigningKey> {
        let key_bytes: [u8; 32] = bytes.try_into()
            .map_err(|_| anyhow!("Invalid signing key length"))?;
        Ok(SigningKey::from_bytes(&key_bytes))
    }
    
    pub fn import_verifying_key(&self, bytes: &[u8]) -> Result<VerifyingKey> {
        let key_bytes: [u8; 32] = bytes.try_into()
            .map_err(|_| anyhow!("Invalid verifying key length"))?;
        VerifyingKey::from_bytes(&key_bytes)
            .map_err(|e| anyhow!("Failed to import verifying key: {}", e))
    }
}