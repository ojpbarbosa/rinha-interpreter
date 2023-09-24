use std::fs;

use rinha_interpreter::{ast, interpreter};

fn main() {
    let file_path = "var/rinha/source.rinha.json";

    let file_buffer = fs::read_to_string(file_path).unwrap();
    let program = serde_json::from_str::<ast::File>(&file_buffer).unwrap();

    interpreter::interpret_program(program);
}
