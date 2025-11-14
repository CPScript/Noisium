use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

pub fn audio_qrng(num_bits: usize) -> Result<Vec<u8>> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow!("No microphone available"))?;
    
    let config = device.default_input_config()?;
    let sample_format = config.sample_format();
    let config: cpal::StreamConfig = config.into();
    
    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_clone = samples.clone();
    
    let duration_sec = (num_bits as f32 / config.sample_rate.0 as f32 * 1.5).max(0.5);
    
    let stream = match sample_format {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data: &[i16], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend_from_slice(data);
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?,
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data: &[u16], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend(data.iter().map(|&s| s as i16));
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend(data.iter().map(|&s| (s * 32767.0) as i16));
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?,
        _ => return Err(anyhow!("Unsupported sample format: {:?}", sample_format)),
    };
    
    stream.play()?;
    thread::sleep(Duration::from_secs_f32(duration_sec));
    drop(stream);
    
    let samples = samples.lock().unwrap();
    let mut bits = Vec::with_capacity(num_bits);
    
    for &sample in &*samples {
        bits.push((sample & 1) as u8);
        if bits.len() >= num_bits {
            break;
        }
    }
    
    if bits.len() < num_bits {
        return Err(anyhow!("Insufficient audio data collected"));
    }
    
    Ok(bits[..num_bits].to_vec())
}