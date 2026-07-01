# Chipper
A Chip-8 emulator written in Rust, with a desktop frontend built on Raylib. Mostly a project I started to refamiliarize myself with Rust.

## Building
 
Requires [Rust](https://rustup.rs/).
 
```
git clone https://github.com/yourusername/chip8-emulator
cd chip8-emulator
cargo build --release -p chip8-desktop
```
 
---
 
## Running
 
```
cargo run --release -p chip8-desktop -- <path/to/rom.ch8> [sound.wav] [ticks_per_frame]
```
 
| Argument | Required | Description |
|---|---|---|
| `rom.ch8` | yes | Path to a CHIP-8 ROM file |
| `sound.wav` | no | WAV file to play when the sound timer is active |
| `ticks_per_frame` | no | CPU instructions per frame (default: 10, ~300Hz) |
 
