use std::fs::File;
use std::io::{stdout, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use log::{info, trace};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    trace!("starting up");
    let start = Instant::now();

    let args = Cli::parse();
    let file_reader = get_file_reader(&args.path)?;

    csvp::find_matches(file_reader, &args.pattern, &mut stdout())?;

    print_elapsed_time(start);
    Ok(())
}

// Using the BufReader instead of loading all the file into memory at once
fn get_file_reader(path: &PathBuf) -> Result<BufReader<File>> {
    let file =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;

    Ok(BufReader::new(file))
}

fn print_elapsed_time(start: Instant) {
    let elapsed = start.elapsed();
    let miliseconds = elapsed.as_micros() as f32 / 1000.0;
    info!("executed in {}ms", format!("{:.2}", miliseconds));
}
