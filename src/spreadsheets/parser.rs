use crate::spreadsheets::lexer::{CellReference, Token};

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    CellReference(CellReference),
    CellRange {
        start: CellReference,
        end: CellReference,
    },
    Sum {
        args: Vec<Expression>,
    },
    Product {
        args: Vec<Expression>,
    },
    Difference {
        args: Vec<Expression>,
    },
    Quotient {
        args: Vec<Expression>,
    },
    Function {
        name: String,
        args: Vec<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    // implements a recursive descent parser
    pub fn parse(input: &[Token]) -> Result<Expression> {
        let mut parser = Parser {
            tokens: input.to_vec(),
            index: 0,
        };

        let expression = parser.parse_expression()?;

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        let mut expression = self.parse_term()?;

        while self.index < self.tokens.len() {
            match self.tokens[self.index] {
                Token::Plus => {
                    self.index += 1;
                    let right = self.parse_term()?;
                    expression = Expression::Sum {
                        args: vec![expression, right],
                    };
                }
                Token::Minus => {
                    self.index += 1;
                    let right = self.parse_term()?;
                    expression = Expression::Difference {
                        args: vec![expression, right],
                    };
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression> {
        let mut expression = self.parse_factor()?;

        while self.index < self.tokens.len() {
            match self.tokens[self.index] {
                Token::Multiply => {
                    self.index += 1;
                    let right = self.parse_factor()?;
                    expression = Expression::Product {
                        args: vec![expression, right],
                    };
                }
                Token::Divide => {
                    self.index += 1;
                    let right = self.parse_factor()?;
                    expression = Expression::Quotient {
                        args: vec![expression, right],
                    };
                }
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        let expression = match self.tokens[self.index].to_owned() {
            Token::Number(value) => {
                self.index += 1;
                Expression::Number(value)
            }
            Token::String(value) => {
                self.index += 1;
                Expression::String(value)
            }
            Token::CellReference(cell) => {
                self.index += 1;
                Expression::CellReference(cell)
            }
            Token::CellRange { start, end } => {
                self.index += 1;
                Expression::CellRange { start, end }
            }
            Token::Formula(name) => {
                self.index += 1;
                self.parse_function(name)?
            }
            Token::OpenParenthesis => {
                self.index += 1;
                let expression = self.parse_expression()?;

                match self.tokens[self.index] {
                    Token::CloseParenthesis => self.index += 1,
                    _ => {
                        return Err(anyhow!(
                            "Unexpected token in parse_factor. Expected ')'. Got: {:?}",
                            self.tokens[self.index]
                        ))
                    }
                }

                expression
            }
            _ => return Err(anyhow!("Unexpected factor: {:?}", self.tokens[self.index])),
        };

        Ok(expression)
    }

    fn parse_function(&mut self, name: String) -> Result<Expression> {
        match self.tokens[self.index] {
            Token::OpenParenthesis => self.index += 1,
            _ => return Err(anyhow!("Unexpected token in parse_function. Expected '('")),
        }

        let mut args = vec![];

        while self.index < self.tokens.len() {
            match self.tokens[self.index] {
                Token::Comma => {
                    self.index += 1;
                }
                Token::CloseParenthesis => {
                    self.index += 1;
                    break;
                }
                _ => {
                    let expression = self.parse_expression()?;
                    args.push(expression);
                }
            }
        }

        Ok(Expression::Function { name, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_range() {
        let input = vec![
            Token::Formula(String::from("sum")), // Expression::Function
            Token::OpenParenthesis,
            Token::CellRange {
                start: CellReference {
                    name: String::from("A1"),
                    column: String::from("A"),
                    row: 1,
                },
                end: CellReference {
                    name: String::from("B2"),
                    column: String::from("B"),
                    row: 2,
                },
            },
            Token::CloseParenthesis,
            Token::Plus, // Note: Expression::Sum refers to this token
            Token::Number(1.0),
        ];

        let expression = Parser::parse(&input).unwrap();

        match expression {
            Expression::Sum { args } => {
                assert_eq!(args.len(), 2);
                match args[0].to_owned() {
                    Expression::Function { args, name } => {
                        assert_eq!(name, "sum");
                        assert_eq!(args.len(), 1);
                        match args[0].to_owned() {
                            Expression::CellRange { start, end } => {
                                assert_eq!(
                                    start,
                                    CellReference {
                                        name: String::from("A1"),
                                        column: String::from("A"),
                                        row: 1,
                                    }
                                );
                                assert_eq!(
                                    end,
                                    CellReference {
                                        name: String::from("B2"),
                                        column: String::from("B"),
                                        row: 2,
                                    }
                                );
                            }
                            _ => panic!("Unexpected expression"),
                        }
                    }
                    _ => panic!("Unexpected expression"),
                }
                match args[1].to_owned() {
                    Expression::Number(value) => {
                        assert_eq!(value, 1.0);
                    }
                    _ => panic!("Unexpected expression"),
                }
            }
            _ => panic!("Unexpected expression"),
        }
    }

    #[test]
    fn test_sum_values() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                name: String::from("A1"),
                column: String::from("A"),
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                name: String::from("A2"),
                column: String::from("A"),
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Minus,
            Token::CopyAndIncrementsFormula,
        ];
    }

    #[test]
    fn test_label_reference() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                name: String::from("A1"),
                column: String::from("A"),
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                name: String::from("A2"),
                column: String::from("A"),
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Plus,
            Token::LabelReference {
                label: String::from("label"),
                n_rows: 2,
            },
        ];
    }

    #[test]
    fn test_multiple_label_references() {
        let input = vec![
            Token::Formula(String::from("text")),
            Token::OpenParenthesis,
            Token::Formula(String::from("gte")),
            Token::OpenParenthesis,
            Token::LabelReference {
                label: String::from("adjusted_cost"),
                n_rows: 1,
            },
            Token::Comma,
            Token::LabelReference {
                label: String::from("cost_threshold"),
                n_rows: 1,
            },
            Token::CloseParenthesis,
            Token::CloseParenthesis,
        ];
    }

    #[test]
    fn test_copy_last_result() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                name: String::from("A1"),
                column: String::from("A"),
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                name: String::from("AB2"),
                column: String::from("AB"),
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Plus,
            Token::CopyLastResult {
                column: String::from("A"),
            },
        ];
    }

    #[test]
    fn test_multiple_copy_last_result() {
        let input = vec![
            Token::CopyLastResult {
                column: String::from("E"),
            },
            Token::Plus,
            Token::OpenParenthesis,
            Token::CopyLastResult {
                column: String::from("E"),
            },
            Token::Multiply,
            Token::CellReference(CellReference {
                name: String::from("A9"),
                column: String::from("A"),
                row: 9,
            }),
            Token::CloseParenthesis,
        ];
    }

    #[test]
    fn test_copy_above_result() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                name: String::from("A1"),
                column: String::from("A"),
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                name: String::from("A2"),
                column: String::from("A"),
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Divide,
            Token::CopyAboveResult {
                column: String::from("B"),
            },
        ];
    }

    #[test]
    fn test_tokenize_concat_formula_with_text() {
        let input = vec![
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
        ];
    }

    #[test]
    fn test_tokenize_copy_above_and_nested_formulas() {
        let input = vec![
            Token::CopyAboveResult {
                column: String::from("E"),
            },
            Token::Plus,
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::Formula(String::from("split")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                name: String::from("D3"),
                column: String::from("D"),
                row: 3,
            }),
            Token::Comma,
            Token::String(String::from(",")),
            Token::CloseParenthesis,
            Token::CloseParenthesis,
        ];
    }

    // @todo custom errors instead of panics, so we can test them
    // or use #[should_panic(expected = "assertion failed")]
    // for testing panics
}
