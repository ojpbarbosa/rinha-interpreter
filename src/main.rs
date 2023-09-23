pub mod ast;
pub mod interpreter;

use std::fs;

fn main() {
    let file_path = "var/rinha/source.rinha.json";

    let file_buffer = fs::read_to_string(file_path).unwrap();
    let ast_file: ast::File = serde_json::from_str(&file_buffer).unwrap();

    interpreter::interpret_ast_file(ast_file);
}
