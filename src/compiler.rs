use crate::ast::Expr;
use std::fs::File;
use std::io::Write;

pub fn compile_expression(file: &mut File, expr: Expr) {
    match expr {
        Expr::Number(value) => {
            match writeln!(file, "{}", &format!("PUSH {}", value)) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }
            };
        }
        Expr::Add(box1, box2) => {
            compile_expression(file, *box1);
            compile_expression(file, *box2);
            match writeln!(file, "ADD") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }
            };
        }
        Expr::Subtract(box1, box2) => {
            compile_expression(file, *box1);
            compile_expression(file, *box2);
            match writeln!(file, "SUB") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }
            };
        }
        Expr::Multiply(box1, box2) => {
            compile_expression(file, *box1);
            compile_expression(file, *box2);
            match writeln!(file, "MUL") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }
            };
        }
        Expr::Divide(box1, box2) => {
            compile_expression(file, *box1);
            compile_expression(file, *box2);
            match writeln!(file, "DIV") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    return;
                }
            };
        }
    }
}
