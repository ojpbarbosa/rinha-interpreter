use std::fmt::Display;

use crate::ast::{self};

enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    Tuple((Box<Value>, Box<Value>)),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::Str(str) => write!(f, "{str}"),
            Value::Tuple((first, second)) => write!(f, "({first}, {second})"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            _ => false,
        }
    }
}

fn evaluate(term: ast::Term) -> Result<Value, _> {
    match term {
        ast::Term::Str(str) => Ok(Value::Str(str.value)),
        ast::Term::Print(print) => evaluate_print(print),
        _ => todo!(),
    }
}

fn evaluate_print(print: ast::Print) -> Result<Value, _> {
    let value = evaluate(*print.value);
    println!("{value}");
    Ok(value)
}

pub fn interpret_ast_file(ast_file: ast::File) {
    // todo: create context

    evaluate(ast_file.expression);
}
