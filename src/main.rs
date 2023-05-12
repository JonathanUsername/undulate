mod wav;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wav file on disk or URL for streaming GET request
    #[arg(short, long)]
    path: String,

    /// The number of samples to collapse into a single RMS value
    #[arg(long, default_value_t = 250)]
    rms_range: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let rms_range_window = args.rms_range;

    wav::stream_rms_samples(&args.path, rms_range_window)
}
