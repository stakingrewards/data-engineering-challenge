use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::spreadsheets::cell::Cell;
use crate::spreadsheets::error::{FilesystemError, SpreadsheetError};

const DELIMITER: char = '|';

#[derive(Debug, Clone)]
pub struct Table {
    pub cells: HashMap<String, Cell>,
}

impl Table {
    pub fn from_file(path: &PathBuf) -> Result<Table> {
        let reader = get_file_reader(&path)?;
        build_table(reader)
    }

    pub fn print(&self, writer: &mut impl Write) -> Result<()> {
        for cell in self.cells.values() {
            writeln!(writer, "{}", &cell.value)?;
        }

        Ok(())
    }
}

fn get_file_reader(path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(path).context(FilesystemError::FileNotFound {
        path: path.display().to_string(),
    })?;

    Ok(BufReader::new(file))
}

fn build_table<R: Read>(reader: BufReader<R>) -> Result<Table> {
    let mut row_count = 0;
    let mut column_count = 0;
    let mut table = Table {
        cells: HashMap::new(),
    };

    for line in BufRead::lines(reader) {
        let line = line?;
        let row_cells = line.split(DELIMITER).collect::<Vec<&str>>();

        row_count += 1;
        if table.cells.is_empty() {
            column_count = row_cells.len();
        }

        validate_column_count(row_count, column_count, row_cells.len())?;

        let mut current_row = 1;
        for content in row_cells {
            let cell = Cell::new(row_count, current_row, content.trim().to_string());
            table.cells.insert(cell.hash.clone(), cell);
            current_row += 1;
        }
    }

    Ok(table)
}

fn validate_column_count(
    line: usize,
    expected: usize,
    found: usize,
) -> Result<(), SpreadsheetError> {
    if expected > found {
        return Err(SpreadsheetError::TooManyColumns {
            line,
            expected,
            found,
        });
    } else if expected < found {
        return Err(SpreadsheetError::NotEnoughColumns {
            line,
            expected,
            found,
        });
    }

    Ok(())
}

#[test]
fn accepts_consistent_number_of_columns() {
    let file_contents = "this | is | an | example \n\
                               csv | file | with | the \n\
                               correct | number | of | columns \n";

    let mock_reader = BufReader::new(file_contents.as_bytes());

    let table = build_table(mock_reader).unwrap();

    let mut result = Vec::new();
    table.print(&mut result).unwrap();

    assert_eq!(
        result,
        b"this\nis\nan\nexample\n\
        csv\nfile\nwith\nthe\n\
        correct\nnumber\nof\ncolumns\n"
    );
}

#[test]
fn fails_with_too_many_columns() {
    let file_contents = "this | is | an | example \n\
                               csv | file | with | too | many  \n\
                               columns \n";

    let mock_reader = BufReader::new(file_contents.as_bytes());

    let result = build_table(mock_reader);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(err) => assert_eq!(
            err.to_string(),
            "too many columns in line 2. Expected 4 but found 5"
        ),
    }
}

#[test]
fn fails_with_not_enough_columns() {
    let file_contents = "this | is | an | example \n\
                               csv | file | with \n\
                               not | enough | columns \n";

    let mock_reader = BufReader::new(file_contents.as_bytes());

    let result = build_table(mock_reader);

    match result {
        Ok(_) => panic!("Expected error"),
        Err(err) => assert_eq!(
            err.to_string(),
            "not enough columns in line 2. Expected 4 but found 3"
        ),
    }
}
