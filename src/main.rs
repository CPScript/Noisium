use quantum_crypto_engine::*;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use std::fs;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(author, version, about = "Quantum Cryptography Engine using webcam entropy")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long)]
        output: String,
        
        #[arg(short, long, default_value = "aes256")]
        algorithm: String,
        
        #[arg(short, long)]
        password: Option<String>,
        
        #[arg(short, long, default_value = "webcam")]
        source: String,
    },
    
    Decrypt {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long)]
        output: String,
        
        #[arg(short, long)]
        password: Option<String>,
        
        #[arg(short, long, default_value = "webcam")]
        source: String,
    },
    
    GenerateKey {
        #[arg(short, long)]
        output: String,
        
        #[arg(short, long, default_value = "32")]
        length: usize,
        
        #[arg(short, long, default_value = "webcam")]
        source: String,
    },
    
    Sign {
        #[arg(short, long)]
        message: String,
        
        #[arg(short, long)]
        key: String,
        
        #[arg(short, long)]
        output: String,
        
        #[arg(short, long, default_value = "webcam")]
        source: String,
    },
    
    Verify {
        #[arg(short, long)]
        message: String,
        
        #[arg(short, long)]
        signature: String,
        
        #[arg(short, long)]
        key: String,
        
        #[arg(short, long, default_value = "webcam")]
        source: String,
    },
    
    Status {
        #[arg(short, long, default_value = "webcam")]
        source: String,
        
        #[arg(short, long, default_value = "10")]
        duration: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Encrypt { input, output, algorithm, password, source } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(8192)?;
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            println!("Waiting for entropy pool to fill...");
            wait_for_entropy(&engine, 8192)?;
            
            let engine_arc = Arc::new(engine);
            let cipher_algo = parse_cipher_algorithm(&algorithm)?;
            
            let cipher = if let Some(pwd) = password {
                println!("Using password-based encryption");
                QuantumCipher::with_password(engine_arc.clone(), cipher_algo, &pwd)?
            } else {
                println!("WARNING: No password provided. File cannot be decrypted later!");
                println!("Use --password <your_password> for decryptable files.");
                QuantumCipher::new(engine_arc.clone(), cipher_algo)
            };
            
            println!("Reading input file...");
            let plaintext = fs::read(&input)?;
            
            println!("Encrypting with {} algorithm...", algorithm);
            let ciphertext = cipher.encrypt(&plaintext)?;
            
            fs::write(&output, &ciphertext)?;
            println!("Encrypted data written to {}", output);
            println!("Entropy status: {:?}", engine_arc.health_status());
        },
        
        Commands::Decrypt { input, output, password, source } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(8192)?;
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            println!("Waiting for entropy pool to fill...");
            wait_for_entropy(&engine, 8192)?;
            
            let engine_arc = Arc::new(engine);
            
            println!("Reading encrypted file...");
            let ciphertext = fs::read(&input)?;
            
            let algorithm = if ciphertext[0] == 0x01 {
                CipherAlgorithm::AES256GCM
            } else if ciphertext[0] == 0x02 {
                CipherAlgorithm::ChaCha20Poly1305
            } else {
                anyhow::bail!("Unknown cipher algorithm");
            };
            
            let cipher = if let Some(pwd) = password {
                QuantumCipher::with_password(engine_arc.clone(), algorithm, &pwd)?
            } else {
                anyhow::bail!("Password required for decryption. Use --password <your_password>");
            };
            
            println!("Decrypting...");
            let plaintext = cipher.decrypt(&ciphertext)?;
            
            fs::write(&output, &plaintext)?;
            println!("Decrypted data written to {}", output);
        },
        
        Commands::GenerateKey { output, length, source } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(16384)?; // Made it have a larger pool for key generation
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            println!("Collecting quantum entropy (this may take a moment)...");
            wait_for_entropy(&engine, 8192)?;
            
            let engine_arc = Arc::new(engine);
            let kdf = KeyDerivation::new(engine_arc.clone());
            
            println!("Deriving key material...");
            let key = kdf.derive_key(b"master_key", length)?;
            
            fs::write(&output, hex::encode(&key))?;
            println!("Generated {}-byte key written to {}", length, output);
            
            let stats = engine_arc.entropy_stats();
            println!("Entropy health: {} ({:.4} bits/bit)", stats.health_status, stats.average_entropy);
        },
        
        Commands::Sign { message, key, output, source } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(8192)?;
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            wait_for_entropy(&engine, 8192)?;
            
            let engine_arc = Arc::new(engine);
            let signer = QuantumSigner::new(engine_arc);
            
            let key_bytes = fs::read(&key)?;
            let signing_key = signer.import_signing_key(&key_bytes)?;
            
            let message_bytes = fs::read(&message)?;
            let signature = signer.sign(&message_bytes, &signing_key);
            
            fs::write(&output, signature.to_bytes())?;
            println!("Signature written to {}", output);
        },
        
        Commands::Verify { message, signature, key, source } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(8192)?;
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            wait_for_entropy(&engine, 4096)?;
            
            let engine_arc = Arc::new(engine);
            let signer = QuantumSigner::new(engine_arc);
            
            let key_bytes = fs::read(&key)?;
            let verifying_key = signer.import_verifying_key(&key_bytes)?;
            
            let message_bytes = fs::read(&message)?;
            let signature_bytes = fs::read(&signature)?;
            let sig = ed25519_dalek::Signature::from_bytes(
                &signature_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid signature"))?
            );
            
            match signer.verify(&message_bytes, &sig, &verifying_key) {
                Ok(_) => println!("Signature is VALID"),
                Err(e) => println!("Signature is INVALID: {}", e),
            }
        },
        
        Commands::Status { source, duration } => {
            println!("Initializing quantum entropy collection...");
            let mut engine = QuantumCryptoEngine::new(8192)?;
            let entropy_source = parse_entropy_source(&source)?;
            engine.start_continuous_collection(entropy_source)?;
            
            println!("Monitoring entropy source for {} seconds...\n", duration);
            
            for i in 0..duration {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let stats = engine.entropy_stats();
                
                println!("Time: {}s | Status: {} | Entropy: {:.4} | Available: {}/{} bytes",
                    i + 1,
                    stats.health_status,
                    stats.average_entropy,
                    stats.available_bytes,
                    stats.pool_capacity
                );
            }
            
            println!("\nFinal entropy statistics:");
            let stats = engine.entropy_stats();
            println!("  Health Status: {}", stats.health_status);
            println!("  Average Entropy: {:.4}", stats.average_entropy);
            println!("  Pool Utilization: {:.2}%", 
                (stats.available_bytes as f64 / stats.pool_capacity as f64) * 100.0);
        },
    }
    
    Ok(())
}

fn wait_for_entropy(engine: &QuantumCryptoEngine, min_bits: usize) -> anyhow::Result<()> {
    let max_wait_seconds = 30;
    let check_interval_ms = 200;
    let max_checks = (max_wait_seconds * 1000) / check_interval_ms;
    
    for i in 0..max_checks {
        let stats = engine.entropy_stats();
        
        if stats.available_bytes >= (min_bits / 8) {
            println!("✓ Entropy pool ready ({} bytes collected)", stats.available_bytes);
            return Ok(());
        }
        
        if i % 5 == 0 && i > 0 {
            println!("  Collecting... {}/{} bytes ({:.1}s elapsed)", 
                stats.available_bytes, 
                min_bits / 8,
                (i * check_interval_ms) as f64 / 1000.0
            );
        }
        
        std::thread::sleep(std::time::Duration::from_millis(check_interval_ms));
    }
    
    let stats = engine.entropy_stats();
    anyhow::bail!(
        "Timeout waiting for entropy. Only collected {} bytes in {} seconds. Check if webcam is working.",
        stats.available_bytes,
        max_wait_seconds
    )
}

fn parse_entropy_source(source: &str) -> anyhow::Result<EntropySource> {
    match source.to_lowercase().as_str() {
        "webcam" => Ok(EntropySource::Webcam),
        "audio" => Ok(EntropySource::Audio),
        "hybrid" => Ok(EntropySource::Hybrid),
        _ => anyhow::bail!("Unknown entropy source: {}", source),
    }
}

fn parse_cipher_algorithm(algo: &str) -> anyhow::Result<CipherAlgorithm> {
    match algo.to_lowercase().as_str() {
        "aes256" | "aes" => Ok(CipherAlgorithm::AES256GCM),
        "chacha20" | "chacha" => Ok(CipherAlgorithm::ChaCha20Poly1305),
        _ => anyhow::bail!("Unknown cipher algorithm: {}", algo),
    }
}
