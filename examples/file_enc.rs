use quantum_crypto_engine::*;
use std::sync::Arc;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Quantum Cryptography Engine - File Encryption Example\n");
    
    fs::write("test_plaintext.txt", b"This is a test file for quantum encryption!")?;
    println!("Created test file: test_plaintext.txt");
    
    println!("\nInitializing quantum entropy collection...");
    let mut engine = QuantumCryptoEngine::new(8192)?;
    engine.start_continuous_collection(EntropySource::Webcam)?;
    
    println!("Collecting quantum entropy...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    let engine_arc = Arc::new(engine);
    let cipher = QuantumCipher::new(engine_arc.clone(), CipherAlgorithm::AES256GCM);
    
    println!("\nReading plaintext file...");
    let plaintext = fs::read("test_plaintext.txt")?;
    println!("File size: {} bytes", plaintext.len());
    
    println!("\nEncrypting file...");
    let ciphertext = cipher.encrypt(&plaintext)?;
    fs::write("test_encrypted.bin", &ciphertext)?;
    println!("Encrypted file written to: test_encrypted.bin");
    println!("Encrypted size: {} bytes", ciphertext.len());
    
    println!("\nDecrypting file...");
    let encrypted_data = fs::read("test_encrypted.bin")?;
    let decrypted = cipher.decrypt(&encrypted_data)?;
    fs::write("test_decrypted.txt", &decrypted)?;
    println!("Decrypted file written to: test_decrypted.txt");
    
    let original = fs::read_to_string("test_plaintext.txt")?;
    let recovered = fs::read_to_string("test_decrypted.txt")?;
    
    if original == recovered {
        println!("\nSUCCESS: File encrypted and decrypted correctly!");
        println!("Original:  {}", original);
        println!("Recovered: {}", recovered);
    } else {
        println!("\nERROR: Files do not match!");
    }
    
    println!("\nCleaning up test files...");
    fs::remove_file("test_plaintext.txt")?;
    fs::remove_file("test_encrypted.bin")?;
    fs::remove_file("test_decrypted.txt")?;
    
    Ok(())
}