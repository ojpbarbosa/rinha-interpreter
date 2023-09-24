use std::{env::args, fs, process};

use rinha_interpreter::{ast, interpreter};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(0);
    }

    let file_path = &args[1];

    let file_buffer = match fs::read_to_string(file_path) {
        Ok(buffer) => buffer,
        Err(err) => {
            eprintln!("Error reading {}: {}", file_path, err);
            process::exit(1);
        }
    };

    let program: ast::File = match serde_json::from_str(&file_buffer) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(1);
        }
    };

    interpreter::interpret_program(program);
}
