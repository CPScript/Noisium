use sha3::{Sha3_256, Digest};

pub fn von_neumann_debias(bits: &[u8]) -> Vec<u8> {
    if bits.len() < 2 {
        return bits.to_vec();
    }
    
    let mut result = Vec::with_capacity(bits.len() / 4);
    
    for i in (0..bits.len() - 1).step_by(2) {
        let a = bits[i];
        let b = bits[i + 1];
        
        if a != b {
            result.push(a);
        }
    }
    
    if result.is_empty() && !bits.is_empty() {
        result.push(bits[0]);
    }
    
    result
}

pub fn hash_randomness(bits: &[u8]) -> Vec<u8> {
    if bits.is_empty() {
        return Vec::new();
    }
    
    let bytes = bits_to_bytes(bits);
    
    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    
    let mut result_bits = Vec::with_capacity(result.len() * 8);
    for byte in result {
        for i in 0..8 {
            result_bits.push((byte >> i) & 1);
        }
    }
    
    result_bits
}

pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let padded_len = if bits.len() % 8 != 0 {
        bits.len() + (8 - bits.len() % 8)
    } else {
        bits.len()
    };
    
    let mut padded_bits = bits.to_vec();
    padded_bits.resize(padded_len, 0);
    
    let mut bytes = Vec::with_capacity(padded_len / 8);
    for chunk in padded_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit == 1 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }
    
    bytes
}

pub fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    
    for &byte in bytes {
        for i in 0..8 {
            bits.push((byte >> i) & 1);
        }
    }
    
    bits
}

pub fn estimate_entropy(bits: &[u8]) -> f64 {
    if bits.is_empty() {
        return 0.0;
    }
    
    let ones = bits.iter().filter(|&&b| b == 1).count();
    let zeros = bits.len() - ones;
    
    let p0 = zeros as f64 / bits.len() as f64;
    let p1 = ones as f64 / bits.len() as f64;
    
    let mut entropy = 0.0;
    if p0 > 0.0 {
        entropy -= p0 * p0.log2();
    }
    if p1 > 0.0 {
        entropy -= p1 * p1.log2();
    }
    
    entropy
}