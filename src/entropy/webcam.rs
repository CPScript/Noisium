use anyhow::{Result, anyhow};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType, Resolution};
use nokhwa::Camera;

pub fn webcam_qrng(num_bits: usize) -> Result<Vec<u8>> {
    let requested = RequestedFormat::new::<RgbFormat>(
        RequestedFormatType::AbsoluteHighestResolution
    );
    
    let index = CameraIndex::Index(0);
    let mut camera = Camera::new(index, requested)
        .map_err(|e| anyhow!("Failed to open webcam: {}. Ensure device is connected and accessible", e))?;
    
    camera.open_stream()
        .map_err(|e| anyhow!("Failed to start webcam stream: {}", e))?;
    
    let mut random_bits = Vec::with_capacity(num_bits);
    let mut frames_captured = 0;
    const MAX_FRAMES: usize = 50;
    
    while random_bits.len() < num_bits && frames_captured < MAX_FRAMES {
        match camera.frame() {
            Ok(frame) => {
                frames_captured += 1;
                
                let raw_data = frame.buffer();
                
                for &byte in raw_data {
                    random_bits.push(byte & 1);
                    
                    if random_bits.len() >= num_bits {
                        break;
                    }
                }
            }
            Err(e) => {
                // some frames may fail, that's okay - just skip them :P
                eprintln!("Warning: Failed to capture frame: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
    
    let _ = camera.stop_stream();
    
    if random_bits.len() < num_bits {
        return Err(anyhow!(
            "Failed to collect sufficient entropy from webcam (got {} bits, needed {})",
            random_bits.len(),
            num_bits
        ));
    }
    
    Ok(random_bits[..num_bits].to_vec())
}