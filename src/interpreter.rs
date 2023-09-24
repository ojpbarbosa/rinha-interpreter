use std::fmt::Display;

use crate::ast;

pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    Tuple((Box<Value>, Box<Value>)),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(int) => write!(f, "{int}"),
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

impl ast::Term {
    pub fn location(&self) -> &ast::Location {
        match self {
            ast::Term::Int(t) => &t.location,
            ast::Term::Str(t) => &t.location,
            ast::Term::Bool(t) => &t.location,
            ast::Term::Print(t) => &t.location,
            ast::Term::Binary(t) => &t.location,
            ast::Term::If(t) => &t.location,
            ast::Term::Let(t) => &t.location,
            ast::Term::Var(t) => &t.location,
            ast::Term::Function(t) => &t.location,
            ast::Term::Call(t) => &t.location,
            ast::Term::Tuple(t) => &t.location,
            ast::Term::First(t) => &t.location,
            ast::Term::Second(t) => &t.location,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub location: ast::Location,
    pub kind: RuntimeErrorKind,
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    ArgumentError,
    DivisionByZero,
    UnknowIdentifier(ast::Var),
    InvalidBinaryOperation,
    InvalidNumberOfArguments(ast::Function, ast::Location),
}

fn evaluate(term: ast::Term) -> Result<Value, RuntimeError> {
    match term {
        ast::Term::Int(int) => Ok(Value::Int(int.value)),
        ast::Term::Str(str) => Ok(Value::Str(str.value)),
        ast::Term::Call(call) => evaluate_call(*call),
        ast::Term::Binary(binary) => evaluate_binary(*binary),
        ast::Term::Function(function) => evaluate_function(*function),
        ast::Term::Let(let_) => evaluate_let(*let_),
        ast::Term::If(if_) => evaluate_if(*if_),
        ast::Term::Print(print) => {
            let value = evaluate(print.value)?;
            println!("{value}");
            Ok(value)
        }
        ast::Term::First(first) => match evaluate(first.value)? {
            Value::Tuple((value, _)) => Ok(*value),
            _ => Err(RuntimeError {
                message: String::from("not a tuple"),
                location: first.location,
                kind: RuntimeErrorKind::ArgumentError,
            }),
        },
        ast::Term::Second(second) => match evaluate(second.value)? {
            Value::Tuple((_, value)) => Ok(*value),
            _ => Err(RuntimeError {
                message: String::from("not a tuple"),
                location: second.location,
                kind: RuntimeErrorKind::ArgumentError,
            }),
        },
        ast::Term::Bool(bool) => Ok(Value::Bool(bool.value)),
        ast::Term::Tuple(tuple) => Ok(Value::Tuple((
            Box::new(evaluate(tuple.first)?),
            Box::new(evaluate(tuple.second)?),
        ))),
        ast::Term::Var(var) => evaluate_var(var),
    }
}

fn evaluate_call(call: ast::Call) -> Result<Value, RuntimeError> {
    todo!();
}

fn evaluate_binary(binary: ast::Binary) -> Result<Value, RuntimeError> {
    todo!();
}

fn evaluate_function(function: ast::Function) -> Result<Value, RuntimeError> {
    todo!();
}

fn evaluate_let(let_: ast::Let) -> Result<Value, RuntimeError> {
    todo!();
}

fn evaluate_if(if_: ast::If) -> Result<Value, RuntimeError> {
    todo!();
}

fn evaluate_var(var: ast::Var) -> Result<Value, RuntimeError> {
    todo!();
}

pub fn interpret_program(program: ast::File) {
    // todo: create context
    let _ = evaluate(program.expression);
}
