use crate::ast::Expr;
use crate::lexer::{tokenize, Token};
use crate::parser::{evaluate_expression, parse_expression};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_file(file_path: &str) -> Result<Vec<i32>, Error> {
    let mut result: Vec<i32> = Vec::new();
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", file_path, e);
            return Err(e);
        }
    };

    let file = BufReader::new(file);
    for line in file.lines() {
        match line {
            Ok(content) => {
                let tokenized: Vec<Token> = tokenize(&content);
                let expression: Expr = parse_expression(tokenized);
                let expression_result: i32 = evaluate_expression(&expression);
                result.push(expression_result);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::file_reader::read_file;
    use std::io;

    #[test]
    fn test_file_reader() {
        let test_cases = vec![
            ("non-existent-path.txt", Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))),
            ("test-file.mircea", Ok(vec![3, 1, 152])),
        ];

        for (input, expected) in test_cases {
            let result = read_file(input);
            assert_eq!(result.is_err(), expected.is_err());
            if let Ok(value) = expected {
                assert_eq!(result.unwrap(), value);
            }
        }
    }
}
