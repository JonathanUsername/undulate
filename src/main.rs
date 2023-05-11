mod wav;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wav file
    #[arg(short, long)]
    path: String,

    #[arg(long, default_value_t = 250)]
    pps: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let samples_per_pixel = args.pps;

    wav::stream_rms_samples(&args.path, samples_per_pixel)
}
