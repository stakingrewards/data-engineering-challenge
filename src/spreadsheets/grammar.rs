use crate::spreadsheets::cell::{get_column_name, Cell};
use crate::spreadsheets::lexer::Lexer;
use crate::spreadsheets::parser::Parser;

use anyhow::{anyhow, Result};

use super::table::CellProvider;

trait Functions {
    fn sum(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn gte(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn lte(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn text(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn split(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn concat(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn incfrom(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn copy_last_result(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn copy_above_result(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
    fn copy_and_increments_formula(&self, cell: &Cell, args: &[Expression]) -> Result<Expression>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellReference {
    pub hash: String,
    pub column_name: String,
    pub column: usize,
    pub row: usize,
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
    Sum { args: Vec<Expression> },
    Difference { args: Vec<Expression> },
    Product { args: Vec<Expression> },
    Quotient { args: Vec<Expression> },
    Function { name: String, args: Vec<Expression> },
    Collection { expressions: Vec<Expression> },
}

// provides .to_string() for Expression
impl std::fmt::Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(number) => fmt.write_str(&number.to_string()),
            Expression::String(string) => fmt.write_str(string),
            _ => fmt.write_str("!ERROR!"),
        }
    }
}

impl Functions for Expression {
    fn split(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 2 {
            return Err(anyhow!("split must have exactly 2 arguments"));
        }

        let string = args[0].evaluate(&cell)?;
        let delimiter = args[1].evaluate(&cell)?;

        let string = string.to_string();
        let delimiter = delimiter.to_string();

        let parts = string.split(&delimiter).collect::<Vec<&str>>();

        let mut expressions = Vec::new();
        for part in parts {
            let tokens = Lexer::tokenize(&part);
            let expression = Parser::parse(&tokens)?;

            expressions.push(expression);
        }

        Ok(Expression::Collection { expressions })
    }

    fn sum(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        let mut sum = 0.0;

        for arg in args {
            let mut result;

            if arg.is_collection() {
                result = self.sum(cell, &arg.expressions())?;
            } else {
                result = arg.evaluate(&cell)?;
            }

            if result.is_collection() {
                result = self.sum(cell, &result.expressions())?;
            }

            sum += result.to_number()?;
        }

        Ok(Expression::Number(sum))
    }

    fn gte(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 2 {
            return Err(anyhow!("gte must have exactly 2 arguments"));
        }

        let left = args[0].evaluate(&cell)?;
        let right = args[1].evaluate(&cell)?;

        let left = left.to_number()?;
        let right = right.to_number()?;

        Ok(Expression::String((left >= right).to_string()))
    }

    fn lte(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 2 {
            return Err(anyhow!("lte must have exactly 2 arguments"));
        }

        let left = args[0].evaluate(&cell)?;
        let right = args[1].evaluate(&cell)?;

        let left = left.to_number()?;
        let right = right.to_number()?;

        Ok(Expression::String((left <= right).to_string()))
    }

    // This function does nothing, because our output is always a string.
    // However, it's here for the sake of completeness. It could be used
    // as a formatter, for example to render the table as HTML.
    fn text(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 1 {
            return Err(anyhow!("text must have exactly 1 argument"));
        }

        let arg = args[0].evaluate(&cell)?;

        Ok(Expression::String(format!("{}", arg.to_string())))
    }

    fn concat(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        let mut result = String::new();

        for arg in args {
            let arg = arg.evaluate(&cell)?;
            result.push_str(&arg.to_string());
        }

        Ok(Expression::String(result))
    }

    // this function does not increment, it is just a marker for copy_and_increments_formula
    fn incfrom(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 1 {
            return Err(anyhow!("incfrom must have exactly 1 argument"));
        }

        let arg = args[0].evaluate(&cell)?;
        let value = arg.to_number()?;

        Ok(Expression::Number(value))
    }

    fn copy_above_result(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 1 {
            return Err(anyhow!("copy_above_result must have exactly 1 argument"));
        }

        if cell.row == 1 {
            return Err(anyhow!("copy_above_result cannot be used in the first row"));
        }

        match &args[0] {
            Expression::ColumnReference(column) => {
                let hash = format!("{}{}", column.name, cell.row - 1);
                let table = cell.table.borrow();
                let cell = table
                    .cell(&hash)
                    .expect(format!("cell not found: {}", hash).as_str());

                Ok(Expression::String(cell.result()))
            }
            _ => {
                return Err(anyhow!(
                    "copy_above_result must have a column reference as its first argument"
                ))
            }
        }
    }

    fn copy_last_result(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
        if args.len() != 1 {
            return Err(anyhow!("copy_last_result must have exactly 1 argument"));
        }

        match &args[0] {
            Expression::ColumnReference(column) => {
                let mut row = cell.table.borrow().num_rows;

                while row > 0 {
                    let hash = format!("{}{}", column.name, row);
                    let table = cell.table.borrow();
                    let cell = table
                        .cell(&hash)
                        .expect(format!("cell not found: {}", hash).as_str());

                    if !cell.value.is_empty() {
                        return Ok(Expression::String(cell.result()));
                    }

                    row -= 1;
                }

                Ok(Expression::String(String::new()))
            }
            _ => {
                return Err(anyhow!(
                    "copy_last_result must have a column reference as its first argument"
                ))
            }
        }
    }

    // copies the formula from the cell above, increments row numbers and numbers marked with incfrom
    fn copy_and_increments_formula(&self, cell: &Cell, args: &[Expression]) -> Result<Expression> {
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

        let hash = format!("{}{}", get_column_name(cell.column), cell.row - 1);
        let table = cell.table.borrow();
        let cell_above = table
            .cell(&hash)
            .expect(format!("cell above not found: {}", hash).as_str());

        if cell_above.formula().is_none() {
            return Err(anyhow!(
                "copy_and_increments_formula can only refer to cells with a formula"
            ));
        }

        let new_cell = Cell::new(&cell.table, cell.row, cell.column, &cell_above.value);

        let tokens = Lexer::tokenize_and_increment(&new_cell.value);
        let expression = Parser::parse(&tokens)?;

        expression.evaluate(&new_cell)
    }
}

impl Expression {
    pub fn evaluate(&self, cell: &Cell) -> Result<Expression> {
        match self {
            // Literals
            Expression::Number(number) => Ok(Expression::String(number.to_string())),
            Expression::String(string) => Ok(Expression::String(string.clone())),

            // References
            Expression::CellReference(cell_ref) => {
                let table = cell.table.borrow();
                let cell = table
                    .cell(&cell_ref.hash)
                    .expect(format!("cell not found: {}", cell_ref.hash).as_str());

                Ok(Expression::String(cell.result()))
            }
            Expression::LabelReference(label_ref) => {
                let table = cell.table.borrow();
                let cell = table
                    .cell(&label_ref.label)
                    .expect(format!("label not found: {}", label_ref.label).as_str());

                let mut target_row = cell.row + label_ref.n_rows;

                if target_row > table.num_rows {
                    target_row = table.num_rows;
                }

                let hash = format!("{}{}", get_column_name(cell.column), target_row);
                let cell = table
                    .cell(&hash)
                    .expect(format!("target cell not found: {}", hash).as_str());

                Ok(Expression::String(cell.result()))
            }

            // Operators
            Expression::Sum { args } => {
                let mut result = 0.0;

                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.to_number()?;
                    result += value;
                }

                Ok(Expression::String(result.to_string()))
            }
            Expression::Difference { args } => {
                let result = args[0].evaluate(cell)?;
                let mut result = result.to_number()?;

                for arg in &args[1..] {
                    let value = arg.evaluate(cell)?;
                    let value = value.to_number()?;
                    result -= value;
                }

                Ok(Expression::String(result.to_string()))
            }
            Expression::Product { args } => {
                let mut result = 1.0;

                for arg in args {
                    let value = arg.evaluate(cell)?;
                    let value = value.to_number()?;
                    result *= value;
                }

                Ok(Expression::String(result.to_string()))
            }
            Expression::Quotient { args } => {
                let result = args[0].evaluate(cell)?;
                let mut result = result.to_number()?;

                for arg in &args[1..] {
                    let value = arg.evaluate(cell)?;
                    let value = value.to_number()?;

                    if value == 0.0 {
                        return Err(anyhow!(
                            "it's all fun and games until someone divides by zero"
                        ));
                    }

                    result /= value;
                }

                Ok(Expression::String(result.to_string()))
            }

            // Functions
            Expression::Function { name, args } => match name.as_str() {
                "sum" => self.sum(cell, args),
                "gte" => self.gte(cell, args),
                "lte" => self.lte(cell, args),
                "text" => self.text(cell, args),
                "split" => self.split(cell, args),
                "concat" => self.concat(cell, args),
                "incfrom" => self.incfrom(cell, args),
                "copy_last_result" => self.copy_last_result(cell, args),
                "copy_above_result" => self.copy_above_result(cell, args),
                "copy_and_increments_formula" => self.copy_and_increments_formula(cell, args),
                _ => return Err(anyhow!("unknown function")),
            },

            // Collections
            _ => Err(anyhow!("unexpected expression")),
        }
    }

    fn is_collection(&self) -> bool {
        match self {
            Expression::Collection { expressions: _ } => true,
            _ => false,
        }
    }

    fn expressions(&self) -> Vec<Expression> {
        match self {
            Expression::Collection { expressions } => expressions.clone(),
            _ => vec![self.clone()],
        }
    }

    fn to_number(&self) -> Result<f64> {
        match self {
            Expression::Number(number) => Ok(*number),
            Expression::String(string) => Ok(string.parse::<f64>().unwrap_or(0.0)),
            _ => Err(anyhow!(
                "cannot convert to number: expected number or numeric string"
            )),
        }
    }
}
