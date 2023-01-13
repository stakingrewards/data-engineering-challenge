use std::cell::RefCell;
use std::rc::Rc;

use crate::spreadsheets::calculator::Calculator;
use crate::spreadsheets::table::Table;

#[derive(Debug, Clone)]
pub struct Cell {
    pub table: Rc<RefCell<Table>>,
    pub row: usize,
    pub column: usize,
    pub hash: String,
    pub value: String,
}

const LABEL_PREFIX: char = '!';
const FORMULA_PREFIX: char = '=';

impl Cell {
    pub fn new(table: &Rc<RefCell<Table>>, row: usize, column: usize, value: &str) -> Self {
        let column_name = Self::column_name(column);
        let hash = format!("{}{}", column_name, row);
        let value = value.to_string();

        Cell {
            table: Rc::clone(&table),
            row,
            column,
            hash,
            value,
        }
    }

    pub fn label(&self) -> Option<String> {
        if self.is_label() {
            return Some(self.value[1..].to_string());
        }

        None
    }

    pub fn result(&self) -> String {
        if self.is_formula() {
            return Calculator::calculate(&self)
                .expect(format!("invalid formula: {}", self.value).as_str());
        }

        self.value.clone()
    }

    pub fn formula(&self) -> Option<String> {
        if self.is_formula() {
            return Some(self.value[1..].to_string());
        }

        None
    }

    pub fn column_name(column: usize) -> String {
        let mut column_name = String::new();
        let mut column = column;

        while column > 0 {
            let remainder = (column - 1) % 26;
            let character = char::from_u32('A' as u32 + remainder as u32)
                .expect("invalid character in column name");

            column_name.insert(0, character);
            column = (column - remainder) / 26;
        }

        return format!("{}", column_name);
    }

    pub fn column_number(column: &str) -> usize {
        let mut column_number = 0;
        let mut multiplier = 1;

        for c in column.chars().rev() {
            column_number += (c as usize - 'A' as usize + 1) * multiplier;
            multiplier *= 26;
        }

        return column_number;
    }

    fn is_label(&self) -> bool {
        self.value.starts_with(LABEL_PREFIX)
    }

    fn is_formula(&self) -> bool {
        self.value.starts_with(FORMULA_PREFIX)
    }
}

pub fn get_column_name(column: usize) -> String {
    Cell::column_name(column)
}

pub fn get_column_number(column: &str) -> usize {
    Cell::column_number(column)
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_column_name_assignment() {
        assert_eq!(Cell::column_name(1), "A");
        assert_eq!(Cell::column_name(2), "B");
        assert_eq!(Cell::column_name(3), "C");

        assert_eq!(Cell::column_name(24), "X");
        assert_eq!(Cell::column_name(25), "Y");
        assert_eq!(Cell::column_name(26), "Z");

        assert_eq!(Cell::column_name(27), "AA");
        assert_eq!(Cell::column_name(28), "AB");
        assert_eq!(Cell::column_name(29), "AC");

        assert_eq!(Cell::column_name(50), "AX");
        assert_eq!(Cell::column_name(51), "AY");
        assert_eq!(Cell::column_name(52), "AZ");

        assert_eq!(Cell::column_name(53), "BA");
        assert_eq!(Cell::column_name(54), "BB");
        assert_eq!(Cell::column_name(55), "BC");

        assert_eq!(Cell::column_name(676), "YZ");
        assert_eq!(Cell::column_name(677), "ZA");
        assert_eq!(Cell::column_name(678), "ZB");

        assert_eq!(Cell::column_name(702), "ZZ");
        assert_eq!(Cell::column_name(703), "AAA");
        assert_eq!(Cell::column_name(704), "AAB");

        assert_eq!(Cell::column_name(18278), "ZZZ");
        assert_eq!(Cell::column_name(18279), "AAAA");
        assert_eq!(Cell::column_name(18280), "AAAB");
    }

    #[test]
    fn test_column_number_retrieval() {
        assert_eq!(Cell::column_number("A"), 1);
        assert_eq!(Cell::column_number("B"), 2);
        assert_eq!(Cell::column_number("C"), 3);

        assert_eq!(Cell::column_number("X"), 24);
        assert_eq!(Cell::column_number("Y"), 25);
        assert_eq!(Cell::column_number("Z"), 26);

        assert_eq!(Cell::column_number("AA"), 27);
        assert_eq!(Cell::column_number("AB"), 28);
        assert_eq!(Cell::column_number("AC"), 29);

        assert_eq!(Cell::column_number("AX"), 50);
        assert_eq!(Cell::column_number("AY"), 51);
        assert_eq!(Cell::column_number("AZ"), 52);

        assert_eq!(Cell::column_number("BA"), 53);
        assert_eq!(Cell::column_number("BB"), 54);
        assert_eq!(Cell::column_number("BC"), 55);

        assert_eq!(Cell::column_number("YZ"), 676);
        assert_eq!(Cell::column_number("ZA"), 677);
        assert_eq!(Cell::column_number("ZB"), 678);

        assert_eq!(Cell::column_number("ZZ"), 702);
        assert_eq!(Cell::column_number("AAA"), 703);
        assert_eq!(Cell::column_number("AAB"), 704);

        assert_eq!(Cell::column_number("ZZZ"), 18278);
        assert_eq!(Cell::column_number("AAAA"), 18279);
        assert_eq!(Cell::column_number("AAAB"), 18280);
    }
}
