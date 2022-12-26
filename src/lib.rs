use anyhow::Result;
use std::io::{BufRead, BufReader, Read, Write};

pub fn find_matches<R: Read>(
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
