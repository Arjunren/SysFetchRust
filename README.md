# Sysfetch

## Description
Sysfetch is a lightning-fast command-line utility written in Rust that fetches and displays vital system information. Inspired by neofetch, it pairs clean ASCII art with your OS, kernel, CPU, and RAM metrics. Built with the sysinfo crate, it is fully cross-platform, memory-safe, and executes instantly to give you a beautiful terminal overview.

## Requirements
- Rust and Cargo (Install via [rustup](https://rustup.rs/))

## Setup
Extract the ZIP file and navigate to the project directory:
```bash
cd sysfetch
```

## Usage
Compile and run the fetcher using Cargo:
```bash
cargo run
```

*Note: On the first run, Cargo will download and compile the `sysinfo` dependency. Subsequent runs will execute instantly.*

## Building for Production
To compile a fast, standalone executable:
```bash
cargo build --release
```
The compiled binary will be available at `./target/release/sysfetch`. You can move this binary to `/usr/local/bin` (Linux/macOS) or add it to your system PATH to run it globally!
