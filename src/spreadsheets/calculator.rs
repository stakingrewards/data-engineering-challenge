use crate::spreadsheets::cell::Cell;
use crate::spreadsheets::lexer::Lexer;
use crate::spreadsheets::parser::Parser;

use anyhow::Result;

pub struct Calculator;

impl Calculator {
    pub fn calculate(cell: &Cell) -> Result<String> {
        let tokens = Lexer::tokenize(&cell.value);
        let expression = Parser::parse(&tokens).unwrap();

        expression.evaluate(cell)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use crate::spreadsheets::cell::Cell;
    use crate::spreadsheets::table::Table;

    fn mock_table() -> Rc<RefCell<Table>> {
        //                    A              B            C            D
        let contents = "incFrom(1)   | 3.0        | !total     | !total_plus_1   \n\
                              ^^           | A1+B^      | A1+B^v     | 1.0             \n\
                              sum(A1,A2)   | sum(A1:B2) | sum(A3,B3) | @total<2> + 1.0 \n";

        Table::from_string(contents).unwrap()
    }

    #[test]
    fn test_calculate_sum_with_range() {
        let cell = Cell::new(&mock_table(), 0, 0, "=sum(A1:B2) + 1");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("11.0")); // 1.0 + 2.0 + 3.0 + 4.0 + 1
    }

    #[test]
    fn test_calculate_sum_with_parameters_and_above_formula() {
        let cell = Cell::new(&mock_table(), 3, 4, "=sum(A1,A2)-^^");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("2.0")); // 1.0 + 2.0 - 1.0
    }

    #[test]
    fn test_calculate_sum_with_label_reference() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum(A1, A2)+@total_plus_1<1>");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("4.0"));
    }

    #[test]
    fn test_calculate_gte_with_two_label_references() {
        let cell = Cell::new(
            &mock_table(),
            4,
            1,
            "=text(gte(@adjusted_cost<1>, @cost_threshold<1>))",
        );
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_copy_last_result() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum( A1,AB2)+A^v");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_copy_last_result_twice() {
        let cell = Cell::new(&mock_table(), 4, 1, "=E^v+(E^v*A9)");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_divided_by_copy_above_result() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum(A1,A2)/B^");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_concat_with_inc() {
        let cell = Cell::new(&mock_table(), 4, 1, "=concat(\"t_\", text(incFrom(1)))");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }

    #[test]
    fn test_calculate_sum_with_split() {
        let cell = Cell::new(&mock_table(), 4, 1, "=E^+sum(split(D3, \",\"))");
        let result = Calculator::calculate(&cell).unwrap();

        assert_eq!(result, String::from("3"));
    }
}
