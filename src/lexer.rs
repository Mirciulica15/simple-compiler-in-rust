#[derive(Debug, PartialEq)]
pub enum Token {
    NUMBER(i32),
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    IDENTIFIER,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    input
        .split_whitespace()
        .map(|word| {
            if word == "+" {
                Token::PLUS
            } else if word == "-" {
                Token::MINUS
            } else if word == "*" {
                Token::MULTIPLY
            } else if word == "/" {
                Token::DIVIDE
            } else if let Ok(num) = word.parse::<i32>() {
                Token::NUMBER(num)
            } else {
                Token::IDENTIFIER
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use crate::lexer::{tokenize, Token};

    #[test]
    fn test_tokenize() {
        let test_cases = vec![
            ("1 + 2", vec![Token::NUMBER(1), Token::PLUS, Token::NUMBER(2)]),
            ("5 - 3", vec![Token::NUMBER(5), Token::MINUS, Token::NUMBER(3)]),
            ("42", vec![Token::NUMBER(42)]),
            ("a + b", vec![Token::IDENTIFIER, Token::PLUS, Token::IDENTIFIER]),
            ("5 * 3", vec![Token::NUMBER(5), Token::MULTIPLY, Token::NUMBER(3)]),
            ("100 / 5", vec![Token::NUMBER(100), Token::DIVIDE, Token::NUMBER(5)]),
        ];
        for (input, expected) in test_cases {
            let tokenized = tokenize(input);
            assert_eq!(tokenized, expected);
        }
    }
}
