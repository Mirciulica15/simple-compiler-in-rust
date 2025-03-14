use crate::file_reader::{read_file_and_compile, read_file_and_evaluate_expressions};
use crate::file_writer::write_file;
use std::{env, process};

mod lexer;
mod ast;
mod parser;
mod file_reader;
mod compiler;
mod file_writer;

fn main() {
    println!("Hello, Compiler!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: <input_file> <output_file>");
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let result: Vec<i32> = match read_file_and_evaluate_expressions(input_file) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            return;
        }
    };

    println!("Final result: {:#?}", &result);

    let mut output_file = match write_file(output_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    read_file_and_compile(&mut output_file, input_file);
}
