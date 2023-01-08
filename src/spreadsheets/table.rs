use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::spreadsheets::cell::Cell;
use crate::spreadsheets::error::{FilesystemError, SyntaxError};

const DELIMITER: char = '|';

#[derive(Debug, Clone)]
pub struct Table {
    pub cells: Vec<Cell>,
    pub cells_map: HashMap<String, usize>,
    pub column_widths: Vec<usize>,
    pub num_columns: usize,
    pub num_rows: usize,
}

impl Default for Table {
    fn default() -> Table {
        Table {
            cells: Vec::new(),
            cells_map: HashMap::new(),
            column_widths: Vec::new(),
            num_columns: 0,
            num_rows: 0,
        }
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            ..Default::default()
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Table> {
        let mut table = Table::new();
        let reader = Self::get_file_reader(&path)?;

        table.fill(reader)?;

        Ok(table)
    }

    pub fn print(&self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer)?;

        for cell in self.cells.iter() {
            if cell.column == self.num_columns {
                write!(writer, " {}{}", cell.value, "\n")?;
            } else {
                let column_width = self.column_widths[cell.column];
                let spaces = " ".repeat(column_width - cell.value.len());
                write!(writer, " {}{} {}", cell.value, spaces, DELIMITER)?;
            }
        }

        writeln!(writer)?;
        Ok(())
    }

    fn get_file_reader(path: &PathBuf) -> Result<BufReader<File>> {
        let file = File::open(path).context(FilesystemError::FileNotFound {
            path: path.display().to_string(),
        })?;

        Ok(BufReader::new(file))
    }

    fn fill<R: Read>(&mut self, reader: BufReader<R>) -> Result<()> {
        let mut row = 1;
        for line in BufRead::lines(reader) {
            let line = line?;
            let row_cells_map = line.split(DELIMITER).collect::<Vec<&str>>();

            if self.cells_map.is_empty() {
                self.num_columns = row_cells_map.len();
                self.column_widths = vec![0; self.num_columns + 1];
            }

            Self::validate_column_count(row, self.num_columns, row_cells_map.len())?;

            let mut column = 1;
            for content in row_cells_map {
                self.add_cell(row, column, content);
                column += 1;
            }
            row += 1;
        }

        self.num_rows = row;

        Ok(())
    }

    fn add_cell(&mut self, row: usize, column: usize, content: &str) {
        let index = self.cells.len();
        let cell = Cell::new(row, column, content.trim());

        self.cells_map.insert(cell.hash.clone(), index);
        if let Some(label) = cell.label.clone() {
            self.cells_map.insert(label, index);
        }

        if self.column_widths[column] < cell.value.len() {
            self.column_widths[column] = cell.value.len();
        }

        self.cells.push(cell);
    }

    fn validate_column_count(
        line: usize,
        expected: usize,
        found: usize,
    ) -> Result<(), SyntaxError> {
        if expected < found {
            return Err(SyntaxError::TooManyColumns {
                line,
                expected,
                found,
            });
        } else if expected > found {
            return Err(SyntaxError::NotEnoughColumns {
                line,
                expected,
                found,
            });
        }

        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn accepts_consistent_number_of_columns() {
        let file_contents = "this | is | an | example \n\
                               csv | file | with | the \n\
                               correct | number | of | columns \n";

        let mock_reader = BufReader::new(file_contents.as_bytes());

        let mut table = Table {
            ..Default::default()
        };
        table.fill(mock_reader).unwrap();

        let mut result = Vec::new();
        table.print(&mut result).unwrap();

        assert_eq!(
            result,
            b"this|is|an|example\n\
        csv|file|with|the\n\
        correct|number|of|columns\n"
        );
    }

    #[test]
    fn fails_with_too_many_columns() {
        let file_contents = "this | is | an | example \n\
                               csv | file | with | too | many  \n\
                               columns \n";

        let mock_reader = BufReader::new(file_contents.as_bytes());

        let mut table = Table {
            ..Default::default()
        };
        let result = table.fill(mock_reader);

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

        let mut table = Table {
            ..Default::default()
        };
        let result = table.fill(mock_reader);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(
                err.to_string(),
                "not enough columns in line 2. Expected 4 but found 3"
            ),
        }
    }
}
