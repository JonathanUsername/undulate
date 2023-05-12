# Undulate

## A binary for generating waveform data from a wav file

### How does it work?

It streams a .wav file through [`hound`](https://docs.rs/hound/latest/hound/) to extract samples. In an iterator it then streams out JSON that represents the [RMS](https://manual.audacityteam.org/man/glossary.html#rms) to stdout. This ensures memory usage is very low no matter the size of the source file.

### Examples

```sh
undulate  --path townhall.wav --rms-range 250 > townhall.json
```

### Installation

Download the right binary for your architecture from the Releases tab

### Development

No special requirements, just `cargo build`.
