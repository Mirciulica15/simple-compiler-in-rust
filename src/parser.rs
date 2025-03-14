use crate::ast::Expr;
use crate::lexer::Token;

pub fn parse_expression(tokens: Vec<Token>) -> Expr {
    let mut iter = tokens.iter();
    let mut result = match iter.next() {
        Some(Token::NUMBER(value)) => {
            Expr::Number(*value)
        }
        _ => {
            panic!("Expected a number");
        }
    };

    while let Some(token) = iter.next() {
        match token {
            Token::PLUS => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    let next = iter.next();
                    if let Some(Token::MULTIPLY) = next {
                        if let Some(Token::NUMBER(value2)) = iter.next() {
                            result = Expr::Add(Box::new(result), Box::new(Expr::Multiply(
                                Box::new(Expr::Number(*value)), Box::new(Expr::Number(*value2)))));
                        }
                    } else if let Some(Token::DIVIDE) = next {
                        if let Some(Token::NUMBER(value2)) = iter.next() {
                            result = Expr::Add(Box::new(result), Box::new(Expr::Divide(
                                Box::new(Expr::Number(*value)), Box::new(Expr::Number(*value2)))));
                        }
                    } else {
                        result = Expr::Add(Box::new(result), Box::new(Expr::Number(*value)));
                    }
                }
            }
            Token::MINUS => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    let next = iter.next();
                    if let Some(Token::MULTIPLY) = next {
                        if let Some(Token::NUMBER(value2)) = iter.next() {
                            result = Expr::Subtract(Box::new(result), Box::new(Expr::Multiply(
                                Box::new(Expr::Number(*value)), Box::new(Expr::Number(*value2)))));
                        }
                    } else if let Some(Token::DIVIDE) = next {
                        if let Some(Token::NUMBER(value2)) = iter.next() {
                            result = Expr::Subtract(Box::new(result), Box::new(Expr::Divide(
                                Box::new(Expr::Number(*value)), Box::new(Expr::Number(*value2)))));
                        }
                    } else {
                        result = Expr::Subtract(Box::new(result), Box::new(Expr::Number(*value)));
                    }
                }
            }
            Token::MULTIPLY => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    result = Expr::Multiply(Box::new(result), Box::new(Expr::Number(*value)));
                }
            }
            Token::DIVIDE => {
                if let Some(Token::NUMBER(value)) = iter.next() {
                    result = Expr::Divide(Box::new(result), Box::new(Expr::Number(*value)));
                }
            }
            _ => panic!("Unexpected token")
        }
    }

    result
}

pub fn evaluate_expression(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(box1, box2) => {
            evaluate_expression(box1) + evaluate_expression(box2)
        }
        Expr::Subtract(box1, box2) => {
            evaluate_expression(box1) - evaluate_expression(box2)
        }
        Expr::Multiply(box1, box2) => {
            evaluate_expression(box1) * evaluate_expression(box2)
        }
        Expr::Divide(box1, box2) => {
            evaluate_expression(box1) / evaluate_expression(box2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;
    use crate::ast::Expr::{Add, Divide, Multiply, Number, Subtract};
    use crate::lexer::Token;
    use crate::parser::{evaluate_expression, parse_expression};

    #[test]
    fn test_expression_parser() {
        let test_cases = vec![
            (vec![Token::NUMBER(1), Token::PLUS, Token::NUMBER(2)], Add(Box::new(Number(1)), Box::new(Number(2)))),
            (vec![Token::NUMBER(5), Token::MINUS, Token::NUMBER(3)], Subtract(Box::new(Number(5)), Box::new(Number(3)))),
            (vec![Token::NUMBER(42)], Number(42)),
            (vec![Token::NUMBER(5), Token::MULTIPLY, Token::NUMBER(3)], Multiply(Box::new(Number(5)), Box::new(Number(3)))),
            (vec![Token::NUMBER(100), Token::DIVIDE, Token::NUMBER(5)], Divide(Box::new(Number(100)), Box::new(Number(5)))),
        ];
        for (input, expected) in test_cases {
            let expr: Expr = parse_expression(input);
            assert_eq!(expr, expected);
        }
    }

    #[test]
    fn test_expression_evaluator() {
        let test_cases = vec![
            (Add(Box::new(Number(1)), Box::new(Number(2))), 3),
            (Subtract(Box::new(Number(5)), Box::new(Number(3))), 2),
            (Number(42), 42),
            (Multiply(Box::new(Number(5)), Box::new(Number(3))), 15),
            (Divide(Box::new(Number(140)), Box::new(Number(7))), 20),
        ];
        for (input, expected) in test_cases {
            let result: i32 = evaluate_expression(&input);
            assert_eq!(result, expected);
        }
    }
}
