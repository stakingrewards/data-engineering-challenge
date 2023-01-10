use crate::spreadsheets::lexer::Lexer;
use crate::spreadsheets::parser::{Expression, Parser};

use crate::spreadsheets::cell::{CellReference, ColumnReference, LabelReference};

use anyhow::Result;

pub struct Calculator;

impl Calculator {
    pub fn calculate(input: &str) -> Result<String> {
        let tokens = Lexer::tokenize(input);
        let expression = Parser::parse(&tokens).unwrap();

        Ok(Self::eval(expression))
    }

    fn eval(_expression: Expression) -> String {
        // Evaluate expression and return result
        String::from("1")
    }
}

#[test]
fn test_calculate() {
    assert_eq!(Calculator::calculate("1 + 2").unwrap(), "3.0");
    assert_eq!(Calculator::calculate("1 + 2 * 3").unwrap(), "7.0");
    assert_eq!(Calculator::calculate("(1 + 2) * 3").unwrap(), "9.0");
    assert_eq!(Calculator::calculate("1 + 2 * 3 + 4").unwrap(), "11.0");
    assert_eq!(Calculator::calculate("(1 + 2) * (3 + 4)").unwrap(), "21.0");
}
