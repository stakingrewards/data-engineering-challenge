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
    fn cell(&self, hash: &str) -> Option<&Cell>;
}

struct CellResult {
    result: String,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct Table {
    cells: Vec<Cell>,
    cells_map: HashMap<String, usize>,
    pub num_columns: usize,
    pub num_rows: usize,
}

impl Default for Table {
    fn default() -> Table {
        Table {
            cells: Vec::new(),
            cells_map: HashMap::new(),
            num_columns: 0,
            num_rows: 0,
        }
    }
}

impl CellProvider for Table {
    fn cell(&self, hash: &str) -> Option<&Cell> {
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

    #[cfg(test)]
    pub fn from_string(content: &str) -> Result<Rc<RefCell<Self>>> {
        let table = Table::new();
        let reader = BufReader::new(content.as_bytes());

        Table::fill(&table, reader)?;

        Ok(table)
    }

    pub fn print(&self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer)?;

        let mut width_of = vec![0; self.num_columns + 1];

        let results = self
            .cells
            .iter()
            .map(|cell| {
                let result = cell.result();
                let column = cell.column;
                width_of[column] = width_of[column].max(result.len());

                CellResult { result, column }
            })
            .collect::<Vec<CellResult>>();

        for result in results {
            let CellResult { result, column } = result;

            if column == self.num_columns {
                writeln!(writer, "{}", result)?;
            } else {
                let column_width = width_of[column];
                let spaces = " ".repeat(column_width - result.len());
                write!(writer, "{}{} {} ", result, spaces, DELIMITER)?;
            }
        }

        writeln!(writer)?;
        Ok(())
    }

    fn get_file_reader(path: &PathBuf) -> Result<BufReader<File>> {
        let file = File::open(path).context(format!("file not found: {}", path.display()))?;

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

        table.num_rows = row - 1;

        Ok(())
    }

    fn add_cell(&mut self, cell: Cell) {
        let index = self.cells.len();

        self.cells_map.insert(cell.hash.clone(), index);
        if let Some(label) = cell.label().clone() {
            self.cells_map.insert(label, index);
        }

        self.cells.push(cell);
    }

    fn validate_column_count(line: usize, expected: usize, found: usize) -> Result<()> {
        ensure!(
            expected == found,
            format!(
                "invalid column count on line {}. Expected {} but found {}",
                line, expected, found
            )
        );

        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use crate::spreadsheets::table::Table;

    #[test]
    fn outputs_aligned_columns() {
        let file_contents = "this | is | an | example \n\
        csv | file | with | the \n\
        correct | number | of | columns \n";

        let table = Table::from_string(file_contents).unwrap();
        let table = table.borrow();

        let mut result = Vec::new();
        table.print(&mut result).unwrap();

        assert_eq!(
            std::str::from_utf8(&result).unwrap(),
            "\n\
            this    | is     | an   | example\n\
            csv     | file   | with | the\n\
            correct | number | of   | columns\n\
            \n"
        );
    }

    #[test]
    fn outputs_aligned_results() {
        let file_contents = "=incfrom(999) | results           | will     | align  \n\
                                   =^^           | 1                 | =100+100 |        \n\
                                   1             | =incfrom(0) + 1.0 | 1        |        \n";

        let table = Table::from_string(file_contents).unwrap();
        let table = table.borrow();

        let mut result = Vec::new();
        table.print(&mut result).unwrap();

        assert_eq!(
            std::str::from_utf8(&result).unwrap(),
            "\n\
            999  | results | will | align\n\
            1000 | 1       | 200  | \n\
            1    | 1       | 1    | \n\
            \n"
        );
    }

    #[test]
    fn fails_with_too_many_columns() {
        let file_contents = "this | is | an | example \n\
                                   csv | file | with | too | many  \n\
                                   columns \n";

        let result = Table::from_string(file_contents);

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

        let result = Table::from_string(file_contents);

        match result {
            Ok(_) => panic!("Expected error"),
            Err(err) => assert_eq!(
                err.to_string(),
                "invalid column count on line 2. Expected 4 but found 3"
            ),
        }
    }
}
