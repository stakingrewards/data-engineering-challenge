#[derive(Debug, Clone)]
pub struct Cell {
    pub hash: String,
    pub value: String,
    pub formula: Option<String>,
    pub result: Option<String>,
}

const ALPHABET: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const FORMULA_PREFIX: char = '=';

impl Cell {
    pub fn new(row: usize, column: usize, value: String) -> Self {
        let column_string = Self::assign_column_string(column);
        let hash = format!("{}{}", column_string, row);

        Cell {
            hash,
            value,
            formula: None,
            result: None,
        }
    }

    fn assign_column_string(column: usize) -> String {
        let mut column_letter = String::new();

        if column >= 26 {
            return format!("{}", ALPHABET.chars().nth(column).unwrap());
        }

        // DRAFT BELOW HERE !!! This is untested and probably doesn't work
        // @todo: implement and test this; or limit it to 26 columns
        let mut column = column;

        while column > 0 {
            let remainder = column % 26;
            column_letter.push(ALPHABET.chars().nth(remainder).unwrap());
            column = column / 26;
        }

        return format!("{}", column_letter);
    }
}
