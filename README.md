# Undulate

## A binary for generating waveform data from a wav file

### How does it work?

It streams a .wav file through [`hound`](https://docs.rs/hound/latest/hound/) to extract samples. In an iterator it then streams out JSON that represents the [RMS](https://manual.audacityteam.org/man/glossary.html#rms) to stdout. This ensures memory usage is very low no matter the size of the source file.

### Examples

```sh
undulate  --path townhall.wav --rms-range 250 > townhall.json
```

Serialisation errors are ignored and default to 0. They will be reported on stderr.

### Installation

Download the right binary for your architecture from the Releases tab

### Development

Just `cargo build`.

Remember to cross-compile.
To do that on mac m1 you might need the following:

```sh
brew tap SergioBenitez/osxct
brew install x86_64-unknown-linux-gnu
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --target=x86_64-unknown-linux-gnu
```
