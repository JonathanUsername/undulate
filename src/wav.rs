use anyhow::Result;
use bytes::Buf;
use hound::WavReader;
use reqwest;

use serde::ser::{SerializeSeq, Serializer};

use std::{
    cmp, fs,
    io::{self, BufReader},
};

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

type WavStream = WavReader<BufReader<Box<dyn io::Read>>>;

fn get_source_stream(path: &str) -> Result<WavStream> {
    let stream: Box<dyn io::Read> = match path.starts_with("http") {
        true => {
            let request = reqwest::blocking::get(path)?;
            Box::new(request.bytes()?.reader())
        }
        false => Box::new(fs::File::open(path)?),
    };
    let reader = BufReader::new(stream);
    WavReader::new(reader).map_err(|e| anyhow::anyhow!(e))
}

pub fn stream_rms_samples(path: &str, rms_range_window: u32) -> Result<()> {
    let mut reader: WavStream = get_source_stream(path)?;

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
