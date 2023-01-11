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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum_with_range() {
        let result = Calculator::calculate("=sum(A1:A2) + 1").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_parameters_and_above_formula() {
        let result = Calculator::calculate("=sum(A1,A2)-^^").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_label_reference() {
        let result = Calculator::calculate("=sum(A1, A2)+@label<2>").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_gte_with_two_label_references() {
        let result =
            Calculator::calculate("=text(gte(@adjusted_cost<1>, @cost_threshold<1>))").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_copy_last_result() {
        let result = Calculator::calculate("=sum( A1,AB2)+A^v").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_copy_last_result_twice() {
        let result = Calculator::calculate("=E^v+(E^v*A9)").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_divided_by_copy_above_result() {
        let result = Calculator::calculate("=sum(A1,A2)/B^").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_concat_with_inc() {
        let result = Calculator::calculate("=concat(\"t_\", text(incFrom(1)))").unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_split() {
        let result = Calculator::calculate("=E^+sum(split(D3, \",\"))").unwrap();

        assert_eq!(result, String::from("3"));
    }
}
