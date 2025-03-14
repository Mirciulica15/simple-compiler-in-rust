use crate::ast::Expr;
use crate::lexer::{tokenize, Token};
use crate::parser::{evaluate_expression, parse_expression};

mod lexer;
mod ast;
mod parser;

fn main() {
    println!("Hello, Compiler!");
    let tokenized: Vec<Token> = tokenize("5 - 2");
    let expression: Expr = parse_expression(tokenized);
    let expression_result: i32 = evaluate_expression(&expression);
    println!("{:}", expression_result);
}
