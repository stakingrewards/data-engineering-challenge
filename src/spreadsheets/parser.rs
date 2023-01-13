use crate::spreadsheets::cell::get_column_name;
use crate::spreadsheets::grammar::{CellReference, Expression};
use crate::spreadsheets::lexer::Token;

use anyhow::anyhow;
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
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

                let mut cells = Vec::new();

                let start_row = start.row;
                let start_column = start.column;
                let end_row = end.row;
                let end_column = end.column;

                for row in start_row..=end_row {
                    for column in start_column..=end_column {
                        let hash = format!("{}{}", get_column_name(column), row);

                        let cell = Expression::CellReference(CellReference {
                            hash,
                            column_name: get_column_name(column),
                            column,
                            row,
                        });

                        cells.push(cell);
                    }
                }

                Expression::Collection { expressions: cells }
            }
            Token::LabelReference(label) => {
                self.index += 1;
                Expression::LabelReference(label)
            }
            Token::Formula(name) => {
                self.index += 1;
                self.parse_function(name)?
            }
            Token::CopyAboveResult(column) => {
                self.index += 1;
                let name = String::from("copy_above_result");
                let args = vec![Expression::ColumnReference(column)];
                Expression::Function { name, args }
            }
            Token::CopyLastResult(column) => {
                self.index += 1;
                let name = String::from("copy_last_result");
                let args = vec![Expression::ColumnReference(column)];
                Expression::Function { name, args }
            }
            Token::CopyAndIncrementsFormula => {
                self.index += 1;
                let name = String::from("copy_and_increments_formula");
                let args = vec![];
                Expression::Function { name, args }
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
    use crate::spreadsheets::grammar::{CellReference, ColumnReference, LabelReference};

    #[test]
    fn test_sum_range() {
        let input = vec![
            Token::Formula(String::from("sum")), // Expression::Function
            Token::OpenParenthesis,
            Token::CellRange {
                start: CellReference {
                    hash: String::from("A1"),
                    column_name: String::from("A"),
                    column: 1,
                    row: 1,
                },
                end: CellReference {
                    hash: String::from("B2"),
                    column_name: String::from("B"),
                    column: 2,
                    row: 2,
                },
            },
            Token::CloseParenthesis,
            Token::Plus, // Note: Expression::Sum refers to this token
            Token::Number(1.0),
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Sum {
                args: vec![
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![Expression::Collection {
                            expressions: vec![
                                Expression::CellReference(CellReference {
                                    hash: String::from("A1"),
                                    column_name: String::from("A"),
                                    column: 1,
                                    row: 1,
                                }),
                                Expression::CellReference(CellReference {
                                    hash: String::from("B1"),
                                    column_name: String::from("B"),
                                    column: 2,
                                    row: 1,
                                }),
                                Expression::CellReference(CellReference {
                                    hash: String::from("A2"),
                                    column_name: String::from("A"),
                                    column: 1,
                                    row: 2,
                                }),
                                Expression::CellReference(CellReference {
                                    hash: String::from("B2"),
                                    column_name: String::from("B"),
                                    column: 2,
                                    row: 2,
                                })
                            ],
                        }],
                    },
                    Expression::Number(1.0),
                ],
            }
        );
    }

    #[test]
    fn test_subtract_values_with_copy_and_increment_formula() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                hash: String::from("A1"),
                column_name: String::from("A"),
                column: 1,
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                hash: String::from("A2"),
                column_name: String::from("A"),
                column: 1,
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Minus,
            Token::CopyAndIncrementsFormula,
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Difference {
                args: vec![
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![
                            Expression::CellReference(CellReference {
                                hash: String::from("A1"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 1,
                            }),
                            Expression::CellReference(CellReference {
                                hash: String::from("A2"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 2,
                            }),
                        ],
                    },
                    Expression::Function {
                        name: String::from("copy_and_increments_formula"),
                        args: vec![],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_label_reference() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                hash: String::from("A1"),
                column_name: String::from("A"),
                column: 1,
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                hash: String::from("A2"),
                column_name: String::from("A"),
                column: 1,
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Plus,
            Token::LabelReference(LabelReference {
                label: String::from("label"),
                n_rows: 2,
            }),
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Sum {
                args: vec![
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![
                            Expression::CellReference(CellReference {
                                hash: String::from("A1"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 1,
                            }),
                            Expression::CellReference(CellReference {
                                hash: String::from("A2"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 2,
                            }),
                        ],
                    },
                    Expression::LabelReference(LabelReference {
                        label: String::from("label"),
                        n_rows: 2,
                    }),
                ],
            }
        );
    }

    #[test]
    fn test_multiple_label_references() {
        let input = vec![
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
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Function {
                name: String::from("text"),
                args: vec![Expression::Function {
                    name: String::from("gte"),
                    args: vec![
                        Expression::LabelReference(LabelReference {
                            label: String::from("adjusted_cost"),
                            n_rows: 1,
                        }),
                        Expression::LabelReference(LabelReference {
                            label: String::from("cost_threshold"),
                            n_rows: 1,
                        }),
                    ],
                }],
            }
        );
    }

    #[test]
    fn test_copy_last_result() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                hash: String::from("A1"),
                column_name: String::from("A"),
                column: 1,
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                hash: String::from("AB2"),
                column_name: String::from("AB"),
                column: 28,
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Plus,
            Token::CopyLastResult(ColumnReference {
                name: String::from("A"),
            }),
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Sum {
                args: vec![
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![
                            Expression::CellReference(CellReference {
                                hash: String::from("A1"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 1,
                            }),
                            Expression::CellReference(CellReference {
                                hash: String::from("AB2"),
                                column_name: String::from("AB"),
                                column: 28,
                                row: 2,
                            }),
                        ],
                    },
                    Expression::Function {
                        name: String::from("copy_last_result"),
                        args: vec![Expression::ColumnReference(ColumnReference {
                            name: String::from("A"),
                        })]
                    },
                ],
            }
        );
    }

    #[test]
    fn test_multiple_copy_last_result() {
        let input = vec![
            Token::CopyLastResult(ColumnReference {
                name: String::from("E"),
            }),
            Token::Plus,
            Token::OpenParenthesis,
            Token::CopyLastResult(ColumnReference {
                name: String::from("E"),
            }),
            Token::Multiply,
            Token::CellReference(CellReference {
                hash: String::from("A9"),
                column_name: String::from("A"),
                column: 1,
                row: 9,
            }),
            Token::CloseParenthesis,
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Sum {
                args: vec![
                    Expression::Function {
                        name: String::from("copy_last_result"),
                        args: vec![Expression::ColumnReference(ColumnReference {
                            name: String::from("E"),
                        })],
                    },
                    Expression::Product {
                        args: vec![
                            Expression::Function {
                                name: String::from("copy_last_result"),
                                args: vec![Expression::ColumnReference(ColumnReference {
                                    name: String::from("E"),
                                })],
                            },
                            Expression::CellReference(CellReference {
                                hash: String::from("A9"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 9,
                            }),
                        ],
                    }
                ],
            }
        );
    }

    #[test]
    fn test_copy_above_result_result() {
        let input = vec![
            Token::Formula(String::from("sum")),
            Token::OpenParenthesis,
            Token::CellReference(CellReference {
                hash: String::from("A1"),
                column_name: String::from("A"),
                column: 1,
                row: 1,
            }),
            Token::Comma,
            Token::CellReference(CellReference {
                hash: String::from("A2"),
                column_name: String::from("A"),
                column: 1,
                row: 2,
            }),
            Token::CloseParenthesis,
            Token::Divide,
            Token::CopyAboveResult(ColumnReference {
                name: String::from("B"),
            }),
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Quotient {
                args: vec![
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![
                            Expression::CellReference(CellReference {
                                hash: String::from("A1"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 1,
                            }),
                            Expression::CellReference(CellReference {
                                hash: String::from("A2"),
                                column_name: String::from("A"),
                                column: 1,
                                row: 2,
                            }),
                        ],
                    },
                    Expression::Function {
                        name: String::from("copy_above_result"),
                        args: vec![Expression::ColumnReference(ColumnReference {
                            name: String::from("B"),
                        })],
                    },
                ],
            }
        );
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

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Function {
                name: String::from("concat"),
                args: vec![
                    Expression::String(String::from("t_")),
                    Expression::Function {
                        name: String::from("text"),
                        args: vec![Expression::Function {
                            name: String::from("incfrom"),
                            args: vec![Expression::Number(1.0)],
                        }],
                    },
                ],
            }
        );
    }

    #[test]
    fn test_tokenize_copy_above_result_and_nested_formulas() {
        let input = vec![
            Token::CopyAboveResult(ColumnReference {
                name: String::from("E"),
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
                row: 3,
            }),
            Token::Comma,
            Token::String(String::from(",")),
            Token::CloseParenthesis,
            Token::CloseParenthesis,
        ];

        let expression = Parser::parse(&input).unwrap();

        assert_eq!(
            expression,
            Expression::Sum {
                args: vec![
                    Expression::Function {
                        name: String::from("copy_above_result"),
                        args: vec![Expression::ColumnReference(ColumnReference {
                            name: String::from("E"),
                        })],
                    },
                    Expression::Function {
                        name: String::from("sum"),
                        args: vec![Expression::Function {
                            name: String::from("split"),
                            args: vec![
                                Expression::CellReference(CellReference {
                                    hash: String::from("D3"),
                                    column_name: String::from("D"),
                                    column: 4,
                                    row: 3,
                                }),
                                Expression::String(String::from(",")),
                            ],
                        }],
                    },
                ],
            }
        );
    }

    // @todo custom errors instead of panics, so we can test them
    // or use #[should_panic(expected = "assertion failed")]
    // for testing panics
}
