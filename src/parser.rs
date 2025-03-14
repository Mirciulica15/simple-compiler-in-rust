use crate::ast::Expr;
use crate::lexer::Token;

pub fn parse_expression(tokens: Vec<Token>) -> Expr {
    let mut iter = tokens.iter();
    let mut result = match iter.next() {
        Some(Token::NUMBER(value)) => {
            Expr::Number(*value)
        },
        _ => {
            panic!("Expected a number");
        }
    };

    while let Some(token) = iter.next() {
        match token {
            Token::PLUS => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    result = Expr::Add(Box::new(result), Box::new(Expr::Number(*value)))
                }
            },
            Token::MINUS => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    result = Expr::Subtract(Box::new(result), Box::new(Expr::Number(*value)))
                }
            },
            _ => panic!("Unexpected token"),
        }
    }

    result
}
