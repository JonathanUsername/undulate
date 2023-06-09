use anyhow::Result;
use hound::WavReader;

use serde::ser::{SerializeSeq, Serializer};

use std::fs::File;
use std::{cmp, io};

const MAX_BITS_16: i32 = 32767;
const NORMALIZED_RANGE: [i32; 2] = [0, 20];

// See what the RMS stand for https://manual.audacityteam.org/man/glossary.html#rms
fn calculate_rms(samples: &Vec<i32>) -> f32 {
    let sqr_sum = samples.iter().fold(0.0, |sqr_sum, s| {
        let sample = *s as f32;
        sqr_sum + sample * sample
    });
    (sqr_sum / samples.len() as f32).sqrt()
}

fn normalize_16_bit_rms(mut value: i32) -> i32 {
    // clamp first, should always be positive since rms
    value = cmp::max(0, value);
    value = cmp::min(MAX_BITS_16, value);
    // now between 0 and 32767
    value / (MAX_BITS_16 / NORMALIZED_RANGE[1])
}

pub fn stream_rms_samples(filename: &str, rms_range_window: u32) -> Result<()> {
    let mut reader: WavReader<io::BufReader<File>> = hound::WavReader::open(filename)?;

    let out = std::io::stdout();
    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(None)?;

    let mut count: u32 = 0;
    let mut rms_range: Vec<i32> = Vec::new();

    reader.samples::<i32>().flatten().for_each(|sample| {
        let normalized = normalize_16_bit_rms(sample);
        rms_range.push(normalized);
        count += 1;
        if count == rms_range_window {
            let rms = calculate_rms(&rms_range);
            if let Err(e) = seq.serialize_element(&rms) {
                eprintln!("Silenced error in serialisation: {}", e);
                seq.serialize_element(&0.0).unwrap()
            }
            count = 0;
            rms_range = Vec::new();
        }
    });
    seq.end()?;
    Ok(())
}
