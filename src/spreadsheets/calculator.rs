use crate::spreadsheets::cell::Cell;
use crate::spreadsheets::lexer::Lexer;
use crate::spreadsheets::parser::Parser;

use anyhow::Result;

pub struct Calculator;

impl Calculator {
    pub fn calculate(cell: &Cell) -> Result<String> {
        let tokens = Lexer::tokenize(&cell.value);
        let expression = Parser::parse(&tokens)?;
        let result = expression.evaluate(cell)?;

        Ok(result.to_string())
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
        let contents = "=incFrom(1) | 3.0         | !total      | !total_plus_1    | text,to,split \n\
                              =^^         | =A1+B^      | =A1+B^v     | 1.0              | 1,2,3,4       \n\
                              =sum(A1,A2) | =sum(A1:B2) | =sum(A3,B3) | =@total<2> + 1.0 | 1.0,2.1,3.2,4 \n";

        Table::from_string(contents).unwrap()
    }

    #[test]
    fn test_copy_and_increment_formula() {
        let cell = Cell::new(&mock_table(), 2, 1, "=^^");
        let result = Calculator::calculate(&cell).unwrap();

        // ^^ = incFrom(1) = 2
        assert_eq!(result, String::from("2"));
    }

    #[test]
    fn test_copy_and_increment_cell_references() {
        let cell = Cell::new(&mock_table(), 4, 1, "=^^");
        let result = Calculator::calculate(&cell).unwrap();

        // ^^ = A3 = sum(A1, A2) => increments to sum(A2, A3)
        // A1 = 1, A2 = 2, A3 = 3
        // sum(A2, A3) = 5
        assert_eq!(result, String::from("5"));
    }

    #[test]
    fn test_copy_and_increment_cell_range() {
        let cell = Cell::new(&mock_table(), 4, 2, "=^^");
        let result = Calculator::calculate(&cell).unwrap();

        // ^^ = sum(A1:B2) = sum(A2:B3)
        // sum(A1:A2) = (1 + 2 + 3 + 4) = 10
        // sum(A2:A3) = (2 + 3 + 4 + 10) = 19
        assert_eq!(result, String::from("19"));
    }

    #[test]
    fn test_calculate_sum_with_range() {
        let cell = Cell::new(&mock_table(), 2, 4, "=sum(A1:B2) + 1");
        let result = Calculator::calculate(&cell).unwrap();

        // A1:B2 = (1 + 2 + 3 + 4) = 10
        // 10 + 1 = 11
        assert_eq!(result, String::from("11"));
    }

    #[test]
    fn test_calculate_sum_with_parameters_and_above_formula() {
        let cell = Cell::new(&mock_table(), 3, 4, "=sum(A1,A2)-D^");
        let result = Calculator::calculate(&cell).unwrap();

        // A1 = 1, A2 = 2, total 3,
        // D^ = 1
        // 3 - 1 = 2
        assert_eq!(result, String::from("2"));
    }

    #[test]
    fn test_calculate_sum_with_label_reference() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum(A1, A2)+@total_plus_1<1>");
        let result = Calculator::calculate(&cell).unwrap();

        // A1 = 1, A2 = 2, total 3,
        // @total_plus_1<1> = 1
        // 3 + 1 = 4
        assert_eq!(result, String::from("4"));
    }

    #[test]
    fn test_calculate_gte_with_two_label_references() {
        let cell = Cell::new(
            &mock_table(),
            4,
            1,
            "=text(gte(@total<2>, @total_plus_1<2>))",
        );
        let result = Calculator::calculate(&cell).unwrap();

        // @total<2> = 13, @total_plus_1<2> = 14
        // 13 >= 14 = false
        // text(false) = "false" (string)
        assert_eq!(result, String::from("false"));
    }

    #[test]
    fn test_calculate_sum_with_copy_last_result() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum( A1,B2)+A^v");
        let result = Calculator::calculate(&cell).unwrap();

        // A1 = 1, B2 = 4, total 5,
        // A^v = sum(A1,A2) = 1 + 2 = 3
        // 3 + 5 = 8
        assert_eq!(result, String::from("8"));
    }

    #[test]
    fn test_calculate_copy_last_result_twice() {
        let cell = Cell::new(&mock_table(), 1, 1, "=D^v+(D^v*A2)");
        let result = Calculator::calculate(&cell).unwrap();

        // D^v = sum(A3,B3) + 1
        //   A3 = sum(A1,A2) = 1 + 2 = 3
        //   B3 = sum(A1:B2) = 1 + 2 + 3 + 4 = 10
        // D^v = 3 + 10 + 1 = 14
        // 14 + (14 * 2) = 42
        assert_eq!(result, String::from("42"));
    }

    #[test]
    fn test_calculate_sum_divided_by_copy_above_result() {
        let cell = Cell::new(&mock_table(), 4, 1, "=sum(A1,A2)/B^");
        let result = Calculator::calculate(&cell).unwrap();

        // A1 = 1, A2 = 2, total 3,
        // B^ = 10 (the mock cell is in A4, so B^ is relative to row 4, results in B3)
        // B3 = sum(A1:B2) = 1 + 2 + 3 + 4 = 10
        // 3 / 10 = 0.3
        assert_eq!(result, String::from("0.3"));
    }

    #[test]
    fn test_calculate_concat_with_inc() {
        let cell = Cell::new(&mock_table(), 4, 1, "=concat(\"t_\", text(incFrom(1)))");
        let result = Calculator::calculate(&cell).unwrap();

        // incfrom does not increment
        assert_eq!(result, String::from("t_1"));
    }

    #[test]
    fn test_calculate_sum_with_text_returns_error() {
        let cell = Cell::new(&mock_table(), 3, 1, "=D^+sum(split(E1, \",\"))");
        let result = Calculator::calculate(&cell).unwrap();

        // E1 = split(text,to,split) = "text" "to" "split"
        // sum("text", "to", "split") = 0
        // D^ = 1
        // 1 + 0 = 1
        assert_eq!(result, String::from("1"));
    }

    #[test]
    fn test_calculate_sum_with_split_integers() {
        let cell = Cell::new(&mock_table(), 3, 1, "=D^+sum(split(E2, \",\"))");
        let result = Calculator::calculate(&cell).unwrap();

        // E2 = split(1,2,3,4) = 1 2 3 4
        // sum(1,2,3,4) = 10
        // D^ = 1
        // 1 + 10 = 11
        assert_eq!(result, String::from("11"));
    }

    #[test]
    fn test_calculate_sum_with_split_floats() {
        let cell = Cell::new(&mock_table(), 3, 1, "=D^+sum(split(E3, \",\"))");
        let result = Calculator::calculate(&cell).unwrap();

        // E3 = split(1.0,2.1,3.2,4) = 1.0 2.1 3.2 4
        // sum(1.0,2.1,3.2,4) = 10.3
        // D^ = 1
        // 1 + 10.3 = 11.3
        assert_eq!(result, String::from("11.3"));
    }
}
