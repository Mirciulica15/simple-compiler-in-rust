use crate::file_reader::read_file;

mod lexer;
mod ast;
mod parser;
mod file_reader;

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
}
