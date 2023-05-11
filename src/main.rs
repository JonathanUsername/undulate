mod wav;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use serde::ser::{SerializeSeq, Serializer};
use wav::SampleOverview;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wav file
    #[arg(short, long)]
    path: String,

    #[arg(long, default_value_t = 250)]
    pps: u32,

    #[arg(long, default_value_t = 800)]
    width: u32,
}

/// Stream out json to keep memory footprint lower
pub fn serialize_samples_to_stdout(samples: &Vec<SampleOverview>) -> Result<()> {
    let out = std::io::stdout();
    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(None)?;
    for sample in samples {
        seq.serialize_element(&sample)?;
    }
    seq.end()?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "debug")
        .write_style_or("LOG_STYLE", "always");

    let samples_per_pixel = args.pps;
    let width = 800;

    env_logger::init_from_env(env);
    let samples = wav::extract_rms_samples(&args.path, samples_per_pixel, &width)?;
    serialize_samples_to_stdout(&samples)?;

    Ok(())
}
