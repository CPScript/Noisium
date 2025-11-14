# Quantum Cryptography Engine

A cryptography engine that uses quantum entropy from consumer webcam CMOS sensors. Unlike algorithmic PRNGs, this system extracts randomness from quantum shot noise and thermal fluctuations in semiconductor physics.

> [!WARNING] 
> Disclaimer; **This is an experimental cryptographic system**. While based on sound physical principles, it has not undergone formal security certification. Use at your own risk. For production systems requiring certified cryptography, consult with security professionals and use validated implementations. **This was also tested using a Logi 1080p webcam, on windows 11.**

## Features

- **Quantum Entropy Collection**: Extracts randomness from webcam CMOS sensor quantum noise
- **Multiple Cipher Algorithms**: AES-256-GCM and ChaCha20-Poly1305
- **Digital Signatures**: Ed25519 signature generation and verification
- **Key Derivation**: SHA-3 based KDF for key material generation
- **Health Monitoring**: Real-time entropy quality assessment
- **Continuous Operation**: Background entropy collection with automatic quality control
- **Memory Security**: Automatic zeroization of sensitive key material

## Physical Principles

The system exploits three quantum phenomena:

1. **Shot Noise**: Poisson statistics of photon arrival at CMOS photodiodes
2. **Dark Current**: Quantum tunneling in reverse-biased junctions
3. **Thermal Noise**: Johnson-Nyquist noise from charge carrier agitation

## Installation

### System Dependencies

```bash
# Ubuntu/Debian
sudo apt install libopencv-dev libclang-dev libasound2-dev

# Fedora
sudo dnf install opencv-devel clang-devel alsa-lib-devel

# Arch
sudo pacman -S opencv clang alsa-lib
```

### Build

```bash
git clone https://github.com/CPScript/qce.git
cd qce
cargo build --release
```

## Examples

Run the provided examples:

```bash
# Basic encryption/decryption
cargo run --example basic_enc

# File encryption workflow
cargo run --example file_enc

# Key generation and signatures
cargo run --example key_gen
```

## Usage

#### |> Encrypt a File

```bash
./target/release/qce encrypt \
  --input plaintext.txt \
  --output encrypted.bin \
  --algorithm aes256 \
  --source webcam
```

#### |> Decrypt a File

```bash
./target/release/qce decrypt \
  --input encrypted.bin \
  --output decrypted.txt \
  --source webcam
```

#### |> Generate Cryptographic Key

```bash
./target/release/qce generate-key \
  --output master.key \
  --length 32 \
  --source webcam
```

#### |> Monitor Entropy Quality

```bash
./target/release/qce status \
  --source webcam \
  --duration 30
```

## Hardware Setup

### Optimal Webcam Configuration

1. **Light Isolation**: Cover webcam lens with opaque material (black tape, cardboard box)
2. **Pinhole Aperture**: Create 0.5mm aperture for controlled light exposure
3. **Stability**: Mount webcam in fixed position during entropy collection
4. **Environment**: Dark, temperature-stable location

### Performance Characteristics

| Entropy Source | Bit Rate | Quantum Purity | Setup Complexity |
|----------------|----------|----------------|------------------|
| Webcam         | 2-5 Kbps | High          | Medium           |
| Audio          | 10-20 Kbps | Medium      | Low              |
| Hybrid (both)  | 3-8 Kbps | Very High     | Medium           |

---

## Please take into Consideration;

### Strengths;
- True quantum randomness from fundamental physical processes
- Continuous entropy collection with health monitoring
- Automatic quality control rejects degraded entropy
- Memory-safe implementation with automatic key zeroization
- Multiple independent entropy sources can be combined

### Limitations;
- Depends on hardware quality (CMOS sensor characteristics)
- Environmental factors can affect entropy quality
- **Not certified for government/military cryptographic use**
- **Should be combined with traditional CSPRNGs for defense-in-depth**

### Please; 
- Use hybrid mode (webcam + audio) for critical applications
- Monitor entropy health status continuously
- Combine with existing OS entropy sources
- Regular hardware inspection for sensor degradation
- Consider commercial QRNGs for certified applications

## Testing

The engine includes statistical testing:

```bash
# Run unit tests
cargo test

# Generate entropy samples for external analysis
./target/release/qce generate-key --output test.bin --length 1000000

# Test with dieharder suite
dieharder -a -f test.bin

# NIST statistical test suite
./assess 8000000 < test.bin
```

## Acknowledgments

Based on principles from:
- Quantum random number generation research and [Rust-QRNG](https://github.com/CPScript/Rust-QRNG)
- NIST SP 800-90B entropy assessment
- Applied quantum optics literature
- Modern cryptographic engineering practices

### Example Photos

<details closed>
<summary>Click me to expand!</summary>
  
<img width="680" height="400" alt="image" src="https://github.com/user-attachments/assets/808050f6-f678-4310-ba50-731d7946565a" />
<img width="755" height="422" alt="image" src="https://github.com/user-attachments/assets/fd4322b2-7388-486b-8718-67bc8296df68" />
<img width="1088" height="514" alt="image" src="https://github.com/user-attachments/assets/b1070b37-1e6d-4b03-b698-c20430f68512" />
<img width="500" height="408" alt="image" src="https://github.com/user-attachments/assets/3f47a02e-dd65-49a3-b74f-59d96c609469" />


</details>

---

<div align="center">

[MIT License](https://github.com/CPScript/qce/blob/main/LICENSE)

Developers & Contributers; [CPScript](https://github.com/CPScript)

</div>
