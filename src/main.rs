// src/main.rs
/*
 * Main executable for IoT
 */

use clap::Parser;
use iot::{Result, run};

#[derive(Parser)]
#[command(version, about = "IoT - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
