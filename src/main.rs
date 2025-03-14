use crate::compiler::compile_expression;
use crate::file_reader::read_file;
use crate::file_writer::write_file;
use crate::lexer::tokenize;
use crate::parser::parse_expression;

mod lexer;
mod ast;
mod parser;
mod file_reader;
mod compiler;
mod file_writer;

fn main() {
    println!("Hello, Compiler!");

    let result: Vec<i32> = match read_file("test-file.mircea") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            return;
        }
    };

    println!("Final result: {:#?}", &result);

    let tokenized = tokenize("1 + 2");
    let expression = parse_expression(tokenized);

    let mut output_file = match write_file("output.mircea") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    compile_expression(&mut output_file, expression);
}
