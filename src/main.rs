use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read, Write};
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
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    trace!("starting up");
    let start = Instant::now();

    let args = Cli::parse();
    let file_reader = get_file_reader(&args.path)?;

    find_matches(file_reader, &args.pattern, &mut stdout())?;

    print_elapsed_time(start);
    Ok(())
}

// Using the BufReader instead of loading all the file into memory at once
fn get_file_reader(path: &std::path::PathBuf) -> Result<BufReader<File>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    Ok(BufReader::new(file))
}

fn find_matches<R: Read>(
    reader: BufReader<R>,
    pattern: &str,
    writer: &mut impl Write,
) -> Result<()> {
    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

fn print_elapsed_time(start: Instant) {
    let elapsed = start.elapsed();
    let miliseconds = elapsed.as_micros() as f32 / 1000.0;
    info!("executed in {}ms", format!("{:.2}", miliseconds));
}

fn answer() -> i32 {
    42
}

#[test]
fn print_only_lines_with_pattern() {
    let file_contents = "line containing THIS pattern\n\
    line not containing the pattern\n\
    another line containing THIS pattern";

    let mock_reader = BufReader::new(file_contents.as_bytes());

    let mut result = Vec::new();
    find_matches(mock_reader, "THIS", &mut result).unwrap();

    assert_eq!(
        result,
        b"line containing THIS pattern\n\
        another line containing THIS pattern\n"
    );
}
