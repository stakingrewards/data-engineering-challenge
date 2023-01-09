use crate::spreadsheets::calculator::Calculator;

#[derive(Debug, Clone)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub hash: String,
    pub value: String,
    pub label: Option<String>,
    pub result: Option<String>,
}

const LABEL_PREFIX: char = '!';
const FORMULA_PREFIX: char = '=';

impl Cell {
    pub fn new(row: usize, column: usize, value: &str) -> Self {
        let column_string = assign_column_name(column);
        let hash = format!("{}{}", column_string, row);
        let value = value.to_string();

        Cell {
            row,
            column,
            hash,
            value,
            label: None,
            result: None,
        }
    }

    pub fn calculate(&mut self) {
        self.label = self.get_label();
        self.result = self.get_result();
    }

    fn is_label(&self) -> bool {
        self.value.starts_with(LABEL_PREFIX)
    }

    fn is_formula(&self) -> bool {
        self.value.starts_with(FORMULA_PREFIX)
    }

    fn get_label(&self) -> Option<String> {
        if self.is_label() {
            return Some(self.value[1..].to_string());
        }

        None
    }

    fn get_result(&self) -> Option<String> {
        if self.is_formula() {
            let result = Calculator::calculate(&self.value[1..]).unwrap();
            return Some(result);
        }

        None
    }
}

fn assign_column_name(column: usize) -> String {
    let mut column_name = String::new();
    let mut column = column;

    while column > 0 {
        let remainder = (column - 1) % 26;
        column_name.insert(0, char::from_u32('A' as u32 + remainder as u32).unwrap());
        column = (column - remainder) / 26;
    }

    return format!("{}", column_name);
}

#[test]
fn test_column_name_assignment() {
    assert_eq!(assign_column_name(1), "A");
    assert_eq!(assign_column_name(2), "B");
    assert_eq!(assign_column_name(3), "C");

    assert_eq!(assign_column_name(24), "X");
    assert_eq!(assign_column_name(25), "Y");
    assert_eq!(assign_column_name(26), "Z");

    assert_eq!(assign_column_name(27), "AA");
    assert_eq!(assign_column_name(28), "AB");
    assert_eq!(assign_column_name(29), "AC");

    assert_eq!(assign_column_name(50), "AX");
    assert_eq!(assign_column_name(51), "AY");
    assert_eq!(assign_column_name(52), "AZ");

    assert_eq!(assign_column_name(53), "BA");
    assert_eq!(assign_column_name(54), "BB");
    assert_eq!(assign_column_name(55), "BC");

    assert_eq!(assign_column_name(676), "YZ");
    assert_eq!(assign_column_name(677), "ZA");
    assert_eq!(assign_column_name(678), "ZB");

    assert_eq!(assign_column_name(702), "ZZ");
    assert_eq!(assign_column_name(703), "AAA");
    assert_eq!(assign_column_name(704), "AAB");

    assert_eq!(assign_column_name(18278), "ZZZ");
    assert_eq!(assign_column_name(18279), "AAAA");
    assert_eq!(assign_column_name(18280), "AAAB");
}
