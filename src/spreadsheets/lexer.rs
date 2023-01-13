use std::{iter::Peekable, str::Chars};

use crate::spreadsheets::cell::get_column_number;
use crate::spreadsheets::expression::{CellReference, ColumnReference, LabelReference};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Number(f64),
    String(String),
    Formula(String),

    // (A..Z)n references a cell by position
    CellReference(CellReference), // ex: A1, B2, etc.

    // (A..Z)n:[A..Z]n references a range of cells
    CellRange {
        start: CellReference,
        end: CellReference,
    }, // ex: A1:B2

    // @label<n> references a cell that is n rows below a labelled cell
    LabelReference(LabelReference), // ex: @label<1>

    // (A..Z)^ copies the evaluated result of the cell above in the same column
    CopyAboveResult(ColumnReference), // ex: A^ (without v)

    // (A..Z)^v copies the evaluated result of the last non-empty cell in the column
    CopyLastResult(ColumnReference), // ex: A^v or B^v (with v)

    // ^^ Copies the formula from the cell above in the same column
    CopyAndIncrementsFormula, // ^^
}

pub struct Lexer {
    pub increment: usize,
}

impl Lexer {
    pub fn tokenize(content: &str) -> Vec<Token> {
        Self::_tokenize(content, 0)
    }

    pub fn tokenize_and_increment(content: &str, amount: usize) -> Vec<Token> {
        Self::_tokenize(content, amount)
    }

    fn _tokenize(content: &str, increment: usize) -> Vec<Token> {
        if !content.starts_with('=') {
            return vec![Token::String(content.trim().to_string())];
        }

        let mut tokens = Vec::new();
        let mut chars = content.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '(' => tokens.push(Token::OpenParenthesis),
                ')' => tokens.push(Token::CloseParenthesis),
                ',' => tokens.push(Token::Comma),
                '@' => Self::tokenize_label_reference(&mut chars, &mut tokens),
                '^' => Self::tokenize_copy_and_increment(&mut chars, &mut tokens),
                '"' | '\'' => Self::tokenize_string(&mut chars, &mut tokens, c),
                'A'..='Z' | 'a'..='z' => {
                    Self::tokenize_reference_or_formula(&mut chars, &mut tokens, c, increment)
                }
                '0'..='9' => Self::tokenize_number(&mut chars, &mut tokens, c),
                '!' => panic!("Label identifier is not allowed in formulas"),
                _ => (),
            }
        }

        tokens
    }

    fn tokenize_copy_and_increment(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
        if let Some(&c) = chars.peek() {
            match c {
                '^' => {
                    chars.next();
                    tokens.push(Token::CopyAndIncrementsFormula);
                }
                _ => panic!("Invalid copy and increment simbol, expected ^^."),
            }
        }
    }

    fn tokenize_string(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, quote: char) {
        let mut text = String::new();

        while let Some(c) = chars.next() {
            if c == quote {
                break;
            }
            text.push(c);
        }

        tokens.push(Token::String(text));
    }

    fn tokenize_label_reference(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
        let mut label = String::new();
        let mut n_rows = String::new();
        let mut is_label = true;

        while let Some(c) = chars.next() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    label.push(c);
                }
                '0'..='9' => {
                    if is_label {
                        label.push(c);
                    } else {
                        n_rows.push(c);
                    }
                }
                '<' => is_label = false,
                '>' => break,
                _ => panic!("invalid label reference token"),
            }
        }

        if label == "" || n_rows == "" {
            panic!("empty label reference or number of rows to shift");
        }

        tokens.push(Token::LabelReference(LabelReference {
            label: label.to_lowercase(),
            n_rows: n_rows
                .parse()
                .expect("cannot parse number of rows to shift"),
        }));
    }

    fn tokenize_reference_or_formula(
        chars: &mut Peekable<Chars>,
        tokens: &mut Vec<Token>,
        ch: char,
        increment: usize,
    ) {
        let mut text = String::new();
        let mut column = String::new();
        let mut is_cell_reference = false;

        // We always start with (A..Z)
        text.push(uppercase_char(ch));

        while let Some(&c) = chars.peek() {
            match c {
                // (A..Z) after a text can be either formula or reference
                'A'..='Z' | 'a'..='z' => {
                    text.push(uppercase_char(c));
                    chars.next();
                    if is_cell_reference {
                        panic!("References must end with a number: {}", text);
                    }
                }

                // The presence of a number after a text indicates a reference
                '0'..='9' => {
                    is_cell_reference = true;
                    column = text.clone();
                    text.push(c);
                    chars.next();
                }

                // The presence of '(' after a text indicates a formula
                '(' => {
                    if is_cell_reference {
                        panic!("Formulas cannot contain a number: {}", text);
                    }

                    match text.as_str() {
                        "INCFROM" => {
                            tokens.push(Token::Formula(text.to_lowercase()));

                            if increment > 0 {
                                chars.next();
                                tokens.push(Token::OpenParenthesis);

                                let mut number = String::new();

                                while let Some(&c) = chars.peek() {
                                    match c {
                                        ')' => {
                                            break;
                                        }
                                        '0'..='9' | '.' => {
                                            number.push(c);
                                            chars.next();
                                        }
                                        _ => panic!("Invalid formula: {}", text),
                                    }
                                }

                                let number = number
                                    .parse::<f64>()
                                    .expect("cannot parse number in INCFROM lexical analysis");

                                tokens.push(Token::Number(number + increment as f64));
                            }
                        }
                        "SUM" | "SPLIT" | "GTE" | "LTE" | "TEXT" | "CONCAT" => {
                            tokens.push(Token::Formula(text.to_lowercase()))
                        }
                        _ => panic!("Unknown formula: {}", text),
                    };

                    break;
                }

                // The presence of the ^ symbol after a text indicates a column reference
                '^' => {
                    if is_cell_reference {
                        panic!(
                            "The column copy symbol ^ expects a column without a row number: {}",
                            text
                        );
                    }

                    chars.next();
                    if let Some(&c) = chars.peek() {
                        match c {
                            // The presence of v after ^ inverts the direction of the column reference
                            'v' | 'V' => {
                                chars.next();
                                tokens.push(Token::CopyLastResult(ColumnReference { name: text }));
                            }

                            _ => {
                                tokens.push(Token::CopyAboveResult(ColumnReference { name: text }))
                            }
                        }
                    } else {
                        tokens.push(Token::CopyAboveResult(ColumnReference { name: text }));
                    }

                    break;
                }

                _ => {
                    if is_cell_reference {
                        let mut row = text[column.len()..]
                            .parse()
                            .expect("cannot parse row number in tokenize_reference_or_formula");

                        if increment > 0 {
                            row += 1;
                            text = format!("{}{}", column, row);
                        }

                        let start_cell = CellReference {
                            hash: text,
                            column_name: column.clone(),
                            column: get_column_number(&column),
                            row,
                        };
                        Self::tokenize_cell_or_range(chars, tokens, start_cell, increment);
                        break;
                    }
                }
            }
        }
    }

    fn tokenize_cell_or_range(
        chars: &mut Peekable<Chars>,
        tokens: &mut Vec<Token>,
        start_cell: CellReference,
        increment: usize,
    ) {
        if let Some(&c) = chars.peek() {
            match c {
                ':' => {
                    chars.next();

                    if let Some(&c) = chars.peek() {
                        match c {
                            'A'..='Z' | 'a'..='z' => {
                                tokens.push(Token::CellRange {
                                    start: start_cell,
                                    end: Self::tokenize_cell(chars, increment),
                                });
                            }
                            _ => panic!("Invalid range"),
                        }
                    } else {
                        panic!("Invalid range");
                    }
                }
                _ => {
                    tokens.push(Token::CellReference(start_cell));
                }
            }
        } else {
            tokens.push(Token::CellReference(start_cell));
        }
    }

    fn tokenize_cell(chars: &mut Peekable<Chars>, increment: usize) -> CellReference {
        let mut text = String::new();
        let mut column = String::new();
        let mut is_cell_reference = false;

        while let Some(&c) = chars.peek() {
            match c {
                'A'..='Z' | 'a'..='z' => {
                    text.push(uppercase_char(c));
                    chars.next();
                    if is_cell_reference {
                        panic!("References must end with a number: {}", text);
                    }
                }

                '0'..='9' => {
                    is_cell_reference = true;
                    column = text.clone();
                    text.push(c);
                    chars.next();
                }
                _ => break,
            }
        }

        if is_cell_reference {
            let mut row = text[column.len()..]
                .parse()
                .expect("cannot parse row number in tokenize_cell");

            if increment > 0 {
                row += increment;
                text = format!("{}{}", column, row);
            }

            CellReference {
                hash: text,
                column_name: column.clone(),
                column: get_column_number(&column),
                row,
            }
        } else {
            panic!("Invalid cell reference: {}", text);
        }
    }

    fn tokenize_number(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, c: char) {
        let mut number = String::new();
        number.push(c);

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    number.push(c);
                    chars.next();
                }
                _ => break,
            }
        }

        tokens.push(Token::Number(
            number.parse::<f64>().expect("cannot parse number"),
        ));
    }
}

fn uppercase_char(c: char) -> char {
    c.to_uppercase()
        .collect::<Vec<char>>()
        .first()
        .expect("cannot uppercase char")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let content = String::from("=sum(A1:A2) + 1");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellRange {
                    start: CellReference {
                        hash: String::from("A1"),
                        column_name: String::from("A"),
                        column: 1,
                        row: 1
                    },
                    end: CellReference {
                        hash: String::from("A2"),
                        column_name: String::from("A"),
                        column: 1,
                        row: 2
                    }
                },
                Token::CloseParenthesis,
                Token::Plus,
                Token::Number(1.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_copy_and_increment_formula() {
        let content = String::from("=sum(A1,A2)-^^");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(CellReference {
                    hash: String::from("A1"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 1
                }),
                Token::Comma,
                Token::CellReference(CellReference {
                    hash: String::from("A2"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 2
                }),
                Token::CloseParenthesis,
                Token::Minus,
                Token::CopyAndIncrementsFormula,
            ]
        );
    }

    #[test]
    fn test_tokenize_label_reference_with_number() {
        let content = String::from("=sum(A1, A2)+@label_1<2>");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(CellReference {
                    hash: String::from("A1"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 1
                }),
                Token::Comma,
                Token::CellReference(CellReference {
                    hash: String::from("A2"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 2
                }),
                Token::CloseParenthesis,
                Token::Plus,
                Token::LabelReference(LabelReference {
                    label: String::from("label_1"),
                    n_rows: 2,
                }),
            ]
        );
    }

    #[test]
    fn test_tokenize_multiple_label_references() {
        let content = String::from("=text(gte(@adjusted_cost<1>, @cost_threshold<1>))");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("text")),
                Token::OpenParenthesis,
                Token::Formula(String::from("gte")),
                Token::OpenParenthesis,
                Token::LabelReference(LabelReference {
                    label: String::from("adjusted_cost"),
                    n_rows: 1,
                }),
                Token::Comma,
                Token::LabelReference(LabelReference {
                    label: String::from("cost_threshold"),
                    n_rows: 1,
                }),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn test_tokenize_copy_last_result() {
        let content = String::from("=sum( A1,AB2)+A^v");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(CellReference {
                    hash: String::from("A1"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 1
                }),
                Token::Comma,
                Token::CellReference(CellReference {
                    hash: String::from("AB2"),
                    column_name: String::from("AB"),
                    column: 28,
                    row: 2
                }),
                Token::CloseParenthesis,
                Token::Plus,
                Token::CopyLastResult(ColumnReference {
                    name: String::from("A")
                }),
            ]
        );
    }

    #[test]
    fn test_multiple_copy_last_result() {
        let content = String::from("=E^v+(E^v*A9)");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::CopyLastResult(ColumnReference {
                    name: String::from("E")
                }),
                Token::Plus,
                Token::OpenParenthesis,
                Token::CopyLastResult(ColumnReference {
                    name: String::from("E")
                }),
                Token::Multiply,
                Token::CellReference(CellReference {
                    hash: String::from("A9"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 9
                }),
                Token::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn test_tokenize_copy_above_result() {
        let content = String::from("=sum(A1,A2)/B^");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(CellReference {
                    hash: String::from("A1"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 1
                }),
                Token::Comma,
                Token::CellReference(CellReference {
                    hash: String::from("A2"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 2
                }),
                Token::CloseParenthesis,
                Token::Divide,
                Token::CopyAboveResult(ColumnReference {
                    name: String::from("B")
                }),
            ]
        );
    }

    #[test]
    fn test_tokenize_concat_formula_with_text() {
        let content = String::from("=concat(\"t_\", text(incFrom(1)))");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("concat")),
                Token::OpenParenthesis,
                Token::String(String::from("t_")),
                Token::Comma,
                Token::Formula(String::from("text")),
                Token::OpenParenthesis,
                Token::Formula(String::from("incfrom")),
                Token::OpenParenthesis,
                Token::Number(1.0),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]
        );
    }

    #[test]
    fn test_tokenize_copy_above_and_nested_formulas() {
        let content = String::from("=E^+sum(split(D3, \",\"))");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::CopyAboveResult(ColumnReference {
                    name: String::from("E")
                }),
                Token::Plus,
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::Formula(String::from("split")),
                Token::OpenParenthesis,
                Token::CellReference(CellReference {
                    hash: String::from("D3"),
                    column_name: String::from("D"),
                    column: 4,
                    row: 3
                }),
                Token::Comma,
                Token::String(String::from(",")),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
            ]
        );
    }

    // @todo custom errors instead of panics, so we can test them
}
