use crate::file_reader::{read_file_and_compile, read_file_and_evaluate_expressions};
use crate::file_writer::write_file;

mod lexer;
mod ast;
mod parser;
mod file_reader;
mod compiler;
mod file_writer;

fn main() {
    println!("Hello, Compiler!");

    let result: Vec<i32> = match read_file_and_evaluate_expressions("test-file.mircea") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            return;
        }
    };

    println!("Final result: {:#?}", &result);

    let mut output_file = match write_file("output.mircea") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    read_file_and_compile(&mut output_file, "test-file.mircea");
}
