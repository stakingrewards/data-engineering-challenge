use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    RangeSelector,
    Comma,
    Number(f64),
    Text(String),

    // (A..Z)n references a cell by position
    CellReference(String), // ex: A1, B2, etc.

    // @label<n> references a cell that is n rows below a labelled cell
    LabelReference { label: String, n_rows: usize }, // ex: @label<1>

    // (A..Z)^ copies the evaluated result of the cell above in the same column
    CopyAboveResult { column: String }, // ex: A^ (without v)

    // (A..Z)^v copies the evaluated result of the last non-empty cell in the column
    CopyLastResult { column: String }, // ex: A^v or B^v (with v)

    // ^^ Copies the formula from the cell above in the same column
    CopyAndIncrementsFormula, // ^^

    // formula_name(arg1, arg2, ...) calls a formula
    Formula(String), // =sum(A1:A2) or =sum(A1, A2)
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(content: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let binding = content.to_uppercase();
        let mut chars = binding.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '(' => tokens.push(Token::OpenParenthesis),
                ')' => tokens.push(Token::CloseParenthesis),
                ':' => tokens.push(Token::RangeSelector),
                ',' => tokens.push(Token::Comma),
                '^' => Self::tokenize_copy_and_increment_formula(&mut chars, &mut tokens),
                '@' => Self::tokenize_label_reference(&mut chars, &mut tokens),
                'A'..='Z' => Self::tokenize_formula_or_cell_reference(&mut chars, &mut tokens, c),
                '0'..='9' => Self::tokenize_number(&mut chars, &mut tokens, c),
                _ => (),
            }
        }

        tokens
    }

    fn tokenize_copy_and_increment_formula(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
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

    fn tokenize_label_reference(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
        let mut label = String::new();
        let mut n_rows = String::new();

        while let Some(c) = chars.next() {
            match c {
                'A'..='Z' => {
                    label.push(c);
                }
                '0'..='9' => {
                    n_rows.push(c);
                }
                '<' => (),
                '>' => break,
                _ => panic!("Invalid label reference"),
            }
        }

        if label == "" || n_rows == "" {
            panic!("Invalid label reference");
        }

        tokens.push(Token::LabelReference {
            label: label.to_lowercase(),
            n_rows: n_rows.parse().unwrap(),
        });
    }

    fn tokenize_formula_or_cell_reference(
        chars: &mut Peekable<Chars>,
        tokens: &mut Vec<Token>,
        ch: char,
    ) {
        let mut text = String::new();
        let mut is_reference = false;
        text.push(ch);

        while let Some(&c) = chars.peek() {
            match c {
                'A'..='Z' => {
                    text.push(c);
                    chars.next();
                    if is_reference {
                        panic!("References must end with a number: {}", text);
                    }
                }
                '0'..='9' => {
                    is_reference = true;
                    text.push(c);
                    chars.next();
                }
                '(' => {
                    if is_reference {
                        panic!("Formulas cannot contain a number: {}", text);
                    }

                    match text.as_str() {
                        "SUM" | "SPLIT" | "GTE" | "LTE" | "TEXT" => {
                            tokens.push(Token::Formula(text.to_lowercase()))
                        }
                        _ => panic!("Unknown formula: {}", text),
                    };

                    break;
                }
                '^' => {
                    if is_reference {
                        panic!(
                            "The copy symbol ^ expects a column without a row number: {}",
                            text
                        );
                    }

                    chars.next();
                    if let Some(&c) = chars.peek() {
                        match c {
                            'V' => {
                                chars.next();
                                tokens.push(Token::CopyLastResult { column: text });
                            }
                            _ => tokens.push(Token::CopyAboveResult { column: text }),
                        }
                    } else {
                        tokens.push(Token::CopyAboveResult { column: text });
                    }
                    break;
                }
                _ => {
                    if is_reference {
                        tokens.push(Token::CellReference(text));
                        break;
                    }
                }
            }
        }
    }

    fn tokenize_number(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, c: char) {
        let mut number = String::new();
        number.push(c);

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    number.push(c);
                    chars.next();
                }
                '.' => {
                    number.push(c);
                    chars.next();
                }
                _ => break,
            }
        }

        tokens.push(Token::Number(number.parse::<f64>().unwrap()));
    }
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
                Token::CellReference(String::from("A1")),
                Token::RangeSelector,
                Token::CellReference(String::from("A2")),
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
                Token::CellReference(String::from("A1")),
                Token::Comma,
                Token::CellReference(String::from("A2")),
                Token::CloseParenthesis,
                Token::Minus,
                Token::CopyAndIncrementsFormula,
            ]
        );
    }

    #[test]
    fn test_tokenize_label_reference() {
        let content = String::from("=sum(A1,A2)+@label<2>");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(String::from("A1")),
                Token::Comma,
                Token::CellReference(String::from("A2")),
                Token::CloseParenthesis,
                Token::Plus,
                Token::LabelReference {
                    label: String::from("label"),
                    n_rows: 2,
                },
            ]
        );
    }

    #[test]
    fn test_tokenize_copy_last_result() {
        let content = String::from("=sum(A1,A2)+A^v");
        let tokens = Lexer::tokenize(&content);

        assert_eq!(
            tokens,
            vec![
                Token::Formula(String::from("sum")),
                Token::OpenParenthesis,
                Token::CellReference(String::from("A1")),
                Token::Comma,
                Token::CellReference(String::from("A2")),
                Token::CloseParenthesis,
                Token::Plus,
                Token::CopyLastResult {
                    column: String::from("A")
                },
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
                Token::CellReference(String::from("A1")),
                Token::Comma,
                Token::CellReference(String::from("A2")),
                Token::CloseParenthesis,
                Token::Divide,
                Token::CopyAboveResult {
                    column: String::from("B")
                },
            ]
        );
    }
}
