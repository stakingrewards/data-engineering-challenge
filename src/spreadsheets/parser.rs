use crate::spreadsheets::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Num(i64),
    Sum { args: Vec<Expression> },
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl Parser<'_> {
    pub fn parse(_input: &[Token]) -> Result<Expression, String> {
        Ok(Expression::Num(1))
    }

    fn sum(&mut self, args: &[Expression]) -> Expression {
        Expression::Sum {
            args: args.to_vec(),
        }
    }

    fn number(&mut self, input: &str) -> Expression {
        let num = input.parse::<i64>().unwrap();
        Expression::Num(num)
    }
}
