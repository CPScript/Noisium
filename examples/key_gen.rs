use quantum_crypto_engine::*;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    println!("Quantum Cryptography Engine - Key Generation Example\n");
    
    println!("Initializing quantum entropy collection...");
    let mut engine = QuantumCryptoEngine::new(8192)?;
    engine.start_continuous_collection(EntropySource::Webcam)?;
    
    println!("Collecting quantum entropy...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    let engine_arc = Arc::new(engine);
    
    println!("\n=== Symmetric Key Generation ===");
    let kdf = KeyDerivation::new(engine_arc.clone());
    
    let aes_key = kdf.derive_key(b"aes_encryption", 32)?;
    println!("AES-256 Key (32 bytes): {}", hex::encode(&aes_key));
    
    let hmac_key = kdf.derive_key(b"hmac_authentication", 64)?;
    println!("HMAC Key (64 bytes): {}", hex::encode(&hmac_key[..32]));
    
    println!("\n=== Password Generation ===");
    let password = kdf.generate_password(16, crypto::kdf::PasswordCharset::AlphanumericSymbols)?;
    println!("Random Password (16 chars): {}", password);
    
    let hex_key = kdf.generate_password(32, crypto::kdf::PasswordCharset::Hex)?;
    println!("Hex Key (32 chars): {}", hex_key);
    
    println!("\n=== Asymmetric Key Generation (Ed25519) ===");
    let signer = QuantumSigner::new(engine_arc.clone());
    
    let (signing_key, verifying_key) = signer.generate_keypair()?;
    println!("Signing Key: {}", hex::encode(signer.export_signing_key(&signing_key)));
    println!("Verifying Key: {}", hex::encode(signer.export_verifying_key(&verifying_key)));
    
    println!("\n=== Testing Digital Signature ===");
    let message = b"Quantum-signed message";
    let signature = signer.sign(message, &signing_key);
    println!("Message: {}", String::from_utf8_lossy(message));
    println!("Signature: {}", hex::encode(signature.to_bytes()));
    
    match signer.verify(message, &signature, &verifying_key) {
        Ok(_) => println!("Signature verification: SUCCESS"),
        Err(e) => println!("Signature verification: FAILED ({})", e),
    }
    
    let wrong_message = b"Tampered message";
    match signer.verify(wrong_message, &signature, &verifying_key) {
        Ok(_) => println!("Tampered message verification: FAILED (should reject)"),
        Err(_) => println!("Tampered message verification: SUCCESS (correctly rejected)"),
    }
    
    println!("\n=== Entropy Statistics ===");
    let stats = engine_arc.entropy_stats();
    println!("Health Status: {}", stats.health_status);
    println!("Average Entropy: {:.4}", stats.average_entropy);
    println!("Pool Utilization: {:.2}%", 
        (stats.available_bytes as f64 / stats.pool_capacity as f64) * 100.0);
    
    Ok(())
}