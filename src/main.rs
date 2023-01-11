use std::time::Instant;
use std::{io::stdout, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use log::{info, trace};

mod spreadsheets;
use spreadsheets::table::Table;

/// Parses an Excel-like CSV file and prints the result to stdout
#[derive(Debug, Parser)]
struct Cli {
    /// The path to the CSV file to read
    path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    trace!("starting up");
    let start = Instant::now();
    let args = Cli::parse();

    let table = Table::from_file(&args.path)?;
    let table = table.borrow();

    table.print(&mut stdout()).unwrap();

    print_elapsed_time(start);
    Ok(())
}

fn print_elapsed_time(start: Instant) {
    let elapsed = start.elapsed();
    let miliseconds = elapsed.as_micros() as f32 / 1000.0;
    info!("executed in {}ms", format!("{:.2}", miliseconds));
}
