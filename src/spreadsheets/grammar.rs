use crate::spreadsheets::cell::Cell;
use crate::spreadsheets::lexer::Lexer;
use crate::spreadsheets::parser::Parser;

use anyhow::{anyhow, Result};

use super::table::CellProvider;

trait Functions {
    fn sum(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn split(&self, cell: &Cell, args: &[Expression]) -> Result<Vec<Expression>>;
    fn gte(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn lte(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn text(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn concat(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn incfrom(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn copy_above_result(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn copy_last_result(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
    fn copy_and_increments_formula(&self, cell: &Cell, args: &[Expression]) -> Result<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellReference {
    pub hash: String,
    pub column: String,
    pub row: usize, // incrementable
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelReference {
    pub label: String,
    pub n_rows: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnReference {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    CellReference(CellReference),
    LabelReference(LabelReference),
    ColumnReference(ColumnReference),
    CellRange {
        start: CellReference,
        end: CellReference,
    },
    Sum {
        args: Vec<Expression>,
    },
    Difference {
        args: Vec<Expression>,
    },
    Product {
        args: Vec<Expression>,
    },
    Quotient {
        args: Vec<Expression>,
    },
    Function {
        name: String,
        args: Vec<Expression>,
    },
}

impl Functions for Expression {
    fn sum(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        let mut sum = 0.0;

        for arg in args {
            let result = arg.evaluate(&cell)?;
            let value = result.parse::<f64>()?;
            sum += value;
        }

        Ok(sum.to_string())
    }

    fn split(&self, cell: &Cell, args: &[Expression]) -> Result<Vec<Expression>> {
        if args.len() != 2 {
            return Err(anyhow!("split must have exactly 2 arguments"));
        }

        let mut expressions = Vec::new();

        let string = args.first().unwrap().evaluate(&cell)?;
        let delimiter = args.last().unwrap().evaluate(&cell)?;

        let parts = string.split(&delimiter).collect::<Vec<&str>>();
        for part in parts {
            expressions.push(Expression::String(part.to_string()));
        }

        Ok(expressions)
    }

    fn gte(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 2 {
            return Err(anyhow!("gte must have exactly 2 arguments"));
        }

        let left = args.first().unwrap().evaluate(&cell)?;
        let right = args.last().unwrap().evaluate(&cell)?;

        let left = left.parse::<f64>()?;
        let right = right.parse::<f64>()?;

        Ok((left >= right).to_string())
    }

    fn lte(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 2 {
            return Err(anyhow!("lte must have exactly 2 arguments"));
        }

        let left = args.first().unwrap().evaluate(&cell)?;
        let right = args.last().unwrap().evaluate(&cell)?;

        let left = left.parse::<f64>()?;
        let right = right.parse::<f64>()?;

        Ok((left <= right).to_string())
    }

    // this is unecessary, but it's here for the sake of completeness
    fn text(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 1 {
            return Err(anyhow!("text must have exactly 1 argument"));
        }

        let arg = args.first().unwrap().evaluate(&cell)?;

        Ok(format!("\"{}\"", arg))
    }

    fn concat(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        let mut result = String::new();

        for arg in args {
            let arg = arg.evaluate(&cell)?;
            result.push_str(&arg);
        }

        Ok(result)
    }

    // this function does not increment, it is just a marker for the copy_and_increments_formula function
    fn incfrom(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 1 {
            return Err(anyhow!("incfrom must have exactly 1 argument"));
        }

        let arg = args.first().unwrap().evaluate(&cell)?;
        let value = arg.parse::<f64>()?;

        Ok(value.to_string())
    }

    fn copy_above_result(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 1 {
            return Err(anyhow!("copy_above_result must have exactly 1 argument"));
        }

        if cell.row == 1 {
            return Err(anyhow!("copy_above_result cannot be used in the first row"));
        }

        match args.first().unwrap() {
            Expression::ColumnReference(column) => {
                let hash = format!("{}{}", column.name, cell.row - 1);
                let table = cell.table.borrow();
                let cell = table.cell(&hash).unwrap();

                Ok(cell.result().unwrap())
            }
            _ => {
                return Err(anyhow!(
                    "copy_above_result must have a column reference as its first argument"
                ))
            }
        }
    }

    fn copy_last_result(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 1 {
            return Err(anyhow!("copy_last_result must have exactly 1 argument"));
        }

        match args.first().unwrap() {
            Expression::ColumnReference(column) => {
                let mut row = cell.table.borrow().num_rows;

                while row > 0 {
                    let hash = format!("{}{}", column.name, row);
                    let table = cell.table.borrow();
                    let cell = table.cell(&hash).unwrap();

                    if !cell.value.is_empty() {
                        return Ok(cell.result().unwrap());
                    }

                    row -= 1;
                }

                Ok(String::new())
            }
            _ => {
                return Err(anyhow!(
                    "copy_last_result must have a column reference as its first argument"
                ))
            }
        }
    }

    // arguments must be empty. It copies the formula from the cell above, increvments row numbers and numbers marked with incfrom
    fn copy_and_increments_formula(&self, cell: &Cell, args: &[Expression]) -> Result<String> {
        if args.len() != 0 {
            return Err(anyhow!(
                "copy_and_increments_formula must have no arguments"
            ));
        }

        if cell.row == 1 {
            return Err(anyhow!(
                "copy_and_increments_formula cannot be used in the first row"
            ));
        }

        let hash = format!("{}{}", cell.column, cell.row - 1);
        let binding = cell.table.borrow();
        let cell_above = binding.cell(&hash).unwrap();

        if cell_above.formula().is_none() {
            return Err(anyhow!(
                "copy_and_increments_formula can only refer to cells with a formula"
            ));
        }

        let formula = cell_above.formula().unwrap();
        let new_cell = Cell::new(&cell.table, cell.row, cell.column, &format!("={}", formula));

        let tokens = Lexer::tokenize_and_increment(&new_cell.value);
        let expression = Parser::parse(&tokens)?;

        expression.evaluate(&new_cell)
    }
}

impl Expression {
    pub fn evaluate(&self, cell: &Cell) -> Result<String> {
        match self {
            Expression::Number(n) => Ok(n.to_string()),
            Expression::String(s) => Ok(s.to_string()),
            Expression::CellReference(cell_ref) => {
                let table = cell.table.borrow();
                let cell = table.cell(&cell_ref.hash).unwrap();
                Ok(cell.value.clone())
            }
            Expression::LabelReference(label_ref) => {
                let mut result = String::new();
                let mut row = label_ref.n_rows;
                while row > 0 {
                    let cell_ref = CellReference {
                        hash: format!("{}{}", label_ref.label, row),
                        column: label_ref.label.clone(),
                        row,
                    };
                    let table = cell.table.borrow();
                    let cell = table.cell(&cell_ref.hash).unwrap();
                    result.push_str(&cell.value);
                    result.push_str(" ");
                    row -= 1;
                }
                Ok(result)
            }
            Expression::CellRange { start, end } => {
                let mut result = String::new();
                let mut row = start.row;
                while row <= end.row {
                    let cell_ref = CellReference {
                        hash: format!("{}{}", start.column, row),
                        column: start.column.clone(),
                        row,
                    };
                    let table = cell.table.borrow();
                    let cell = table.cell(&cell_ref.hash).unwrap();
                    result.push_str(&cell.value);
                    result.push_str(" ");
                    row += 1;
                }
                Ok(result)
            }
            Expression::Sum { args } => {
                let mut result = 0.0;
                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.parse::<f64>()?;
                    result += value;
                }
                Ok(result.to_string())
            }
            Expression::Difference { args } => {
                let mut result = 0.0;
                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.parse::<f64>()?;
                    result -= value;
                }
                Ok(result.to_string())
            }
            Expression::Product { args } => {
                let mut result = 1.0;
                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.parse::<f64>()?;
                    result *= value;
                }
                Ok(result.to_string())
            }
            Expression::Quotient { args } => {
                let mut result = 1.0;
                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.parse::<f64>()?;
                    result /= value;
                }
                Ok(result.to_string())
            }
            Expression::Function { name, args } => {
                let mut result = String::new();
                for arg in args {
                    let value = arg.evaluate(cell)?;
                    result.push_str(&value);
                    result.push_str(" ");
                }
                Ok(result)
            }
            _ => Err(anyhow!("unexpected expression")),
        }
    }
}
