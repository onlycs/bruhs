# BRUHS

Animated image format (like .gif) based on the BRUH file extension.

I actually do some delta frame compression, so thats cool i guess.

## Limitations

- No transparency
- Fixed frame rate (whatever ffmpeg's default is)
- Lossy

## Usage

- `cargo run -- compile [gif]` - .gif to .bruhs
- `cargo run -- decompile [bruhs]` - .bruhs to .gif
- `cargo run -- [bruhs]` - open .bruhs file in default image viewer

## Build

`cargo build --release` will create a `.exe` on Windows or an executable
on Linux in `[project dir]/target/release`. Build it yourself I don't use Windows anymore.
