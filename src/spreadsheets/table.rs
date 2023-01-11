use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{ensure, Context, Result};

use crate::spreadsheets::cell::Cell;

const DELIMITER: char = '|';

pub trait CellProvider: std::fmt::Debug {
    fn get_cell(&self, hash: &str) -> Option<&Cell>;
}

#[derive(Debug, Clone)]
pub struct Table {
    cells: Vec<Cell>,
    cells_map: HashMap<String, usize>,
    column_widths: Vec<usize>,
    num_columns: usize,
}

impl Default for Table {
    fn default() -> Table {
        Table {
            cells: Vec::new(),
            cells_map: HashMap::new(),
            column_widths: Vec::new(),
            num_columns: 0,
        }
    }
}

impl CellProvider for Table {
    fn get_cell(&self, hash: &str) -> Option<&Cell> {
        self.cells_map.get(hash).map(|index| &self.cells[*index])
    }
}

impl Table {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Table {
            ..Default::default()
        }))
    }

    pub fn from_file(path: &PathBuf) -> Result<Rc<RefCell<Self>>> {
        let table = Table::new();
        let reader = Self::get_file_reader(&path)?;

        Table::fill(&table, reader)?;

        Ok(table)
    }

    pub fn print(&self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer)?;

        for cell in self.cells.iter() {
            if cell.column == self.num_columns {
                writeln!(writer, "{}", cell.value)?;
            } else {
                let column_width = self.column_widths[cell.column];
                let spaces = " ".repeat(column_width - cell.value.len());
                write!(writer, "{}{} {} ", cell.value, spaces, DELIMITER)?;
            }
        }

        writeln!(writer)?;
        Ok(())
    }

    fn get_file_reader(path: &PathBuf) -> Result<BufReader<File>> {
        let file =
            File::open(path).with_context(|| format!("File not found: {}", path.display()))?;

        Ok(BufReader::new(file))
    }

    fn fill<R: Read>(rc: &Rc<RefCell<Self>>, reader: BufReader<R>) -> Result<()> {
        let mut table = rc.try_borrow_mut()?;
        let mut row = 1;

        for line in BufRead::lines(reader) {
            let line = line?;
            let row_cells_map = line.split(DELIMITER).collect::<Vec<&str>>();

            if table.cells_map.is_empty() {
                table.num_columns = row_cells_map.len();
                table.column_widths = vec![0; table.num_columns + 1];
            }

            Self::validate_column_count(row, table.num_columns, row_cells_map.len())?;

            let mut column = 1;
            for content in row_cells_map {
                let cell = Cell::new(rc, row, column, content.trim());
                table.add_cell(cell);
                column += 1;
            }
            row += 1;
        }

        Ok(())
    }

    fn add_cell(&mut self, cell: Cell) {
        let index = self.cells.len();

        self.cells_map.insert(cell.hash.clone(), index);
        if let Some(label) = cell.label().clone() {
            self.cells_map.insert(label, index);
        }

        if self.column_widths[cell.column] < cell.value.len() {
            self.column_widths[cell.column] = cell.value.len();
        }

        self.cells.push(cell);
    }

    fn validate_column_count(line: usize, expected: usize, found: usize) -> Result<()> {
        ensure!(
            expected == found,
            "invalid column count on line {}. Expected {} but found {}",
            line,
            expected,
            found
        );

        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn outputs_aligned_columns() {
        let file_contents = "this | is | an | example \n\
                               csv | file | with | the \n\
                               correct | number | of | columns \n";

        let mock_reader = BufReader::new(file_contents.as_bytes());

        let table = Table::new();
        Table::fill(&table, mock_reader).unwrap();

        let mut result = Vec::new();
        table.borrow().print(&mut result).unwrap();

        let _string = String::from_utf8(result.clone()).unwrap();

        assert_eq!(
            result,
            b"\n\
            this    | is     | an   | example\n\
            csv     | file   | with | the\n\
            correct | number | of   | columns\n\
            \n"
        );
    }

    #[test]
    fn fails_with_too_many_columns() {
        let file_contents = "this | is | an | example \n\
                               csv | file | with | too | many  \n\
                               columns \n";

        let mock_reader = BufReader::new(file_contents.as_bytes());

        let table = Table::new();
        let result = Table::fill(&table, mock_reader);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(
                err.to_string(),
                "invalid column count on line 2. Expected 4 but found 5"
            ),
        }
    }

    #[test]
    fn fails_with_not_enough_columns() {
        let file_contents = "this | is | an | example \n\
                               csv | file | with \n\
                               not | enough | columns \n";

        let mock_reader = BufReader::new(file_contents.as_bytes());

        let table = Table::new();
        let result = Table::fill(&table, mock_reader);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(
                err.to_string(),
                "invalid column count on line 2. Expected 4 but found 3"
            ),
        }
    }
}
