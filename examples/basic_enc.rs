use quantum_crypto_engine::*;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    println!("Quantum Cryptography Engine - Basic Encryption Example\n");
    
    println!("Initializing quantum entropy collection from webcam...");
    let mut engine = QuantumCryptoEngine::new(8192)?;
    engine.start_continuous_collection(EntropySource::Webcam)?;
    
    println!("Waiting for entropy pool to fill...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    let stats = engine.entropy_stats();
    println!("Entropy Status: {}", stats.health_status);
    println!("Average Entropy: {:.4}\n", stats.average_entropy);
    
    let engine_arc = Arc::new(engine);
    
    println!("Creating cipher with AES-256-GCM...");
    let cipher = QuantumCipher::new(engine_arc.clone(), CipherAlgorithm::AES256GCM);
    
    let message = b"This is a secret message encrypted with quantum entropy!";
    println!("Original message: {}", String::from_utf8_lossy(message));
    
    println!("\nEncrypting...");
    let ciphertext = cipher.encrypt(message)?;
    println!("Ciphertext length: {} bytes", ciphertext.len());
    println!("Ciphertext (hex): {}", hex::encode(&ciphertext[..32.min(ciphertext.len())]));
    
    println!("\nDecrypting...");
    let decrypted = cipher.decrypt(&ciphertext)?;
    println!("Decrypted message: {}", String::from_utf8_lossy(&decrypted));
    
    if message == decrypted.as_slice() {
        println!("\nSUCCESS: Message encrypted and decrypted correctly!");
    } else {
        println!("\nERROR: Decryption mismatch!");
    }
    
    Ok(())
}