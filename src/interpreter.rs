use std::fmt::Display;

use crate::ast;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub location: ast::Location,
    pub kind: RuntimeErrorKind,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} at {}",
            self.kind, self.message, self.location.start,
        )
    }
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    ArgumentError,
    DivisionByZero,
    InvalidBinaryOperation,
}

impl Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorKind::ArgumentError => write!(f, "Argument Error"),
            RuntimeErrorKind::DivisionByZero => write!(f, "Division by Zero"),
            RuntimeErrorKind::InvalidBinaryOperation => write!(f, "Invalid Binary Operation"),
        }
    }
}

pub fn interpret_program(program: ast::File) {
    // todo: create context
    if let Err(error) = evaluate(program.expression) {
        println!("{}", error);
    }
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
    let lhs = evaluate(binary.lhs)?;
    let rhs = evaluate(binary.rhs)?;

    let location = binary.location;
    match binary.op {
        ast::BinaryOp::Add => lhs.add(&rhs),
        ast::BinaryOp::Sub => lhs.sub(&rhs, location),
        ast::BinaryOp::Mul => lhs.mul(&rhs, location),
        ast::BinaryOp::Div => lhs.div(&rhs, location),
        ast::BinaryOp::Rem => lhs.rem(&rhs, location),
        ast::BinaryOp::Eq => lhs.eq(&rhs, location),
        ast::BinaryOp::Neq => lhs.neq(&rhs, location),
        ast::BinaryOp::Lt => lhs.lt(&rhs, location),
        ast::BinaryOp::Lte => lhs.lte(&rhs, location),
        ast::BinaryOp::Gt => lhs.gt(&rhs, location),
        ast::BinaryOp::Gte => lhs.gte(&rhs, location),
        ast::BinaryOp::And => lhs.and(&rhs, location),
        ast::BinaryOp::Or => lhs.or(&rhs, location),
    }
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

impl Value {
    pub fn add(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (a, b) => Ok(Value::Str(format!("{a}{b}"))),
        }
    }

    pub fn sub(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (a, b) => Err(invalid_numeric_op(String::from("subtract"), a, b, location)),
        }
    }

    pub fn mul(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (a, b) => Err(invalid_numeric_op(String::from("multiply"), a, b, location)),
        }
    }

    pub fn div(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => {
                if b == &0 {
                    Err(divison_by_zero(
                        format!("Cannot divide {} by 0", a),
                        location,
                    ))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (a, b) => Err(invalid_numeric_op(String::from("divide"), a, b, location)),
        }
    }

    pub fn rem(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (&self, &other) {
            (Value::Int(a), Value::Int(b)) => {
                if b == &0 {
                    Err(divison_by_zero(
                        format!("Cannot calculate remainder of {} dividing by 0", a),
                        location,
                    ))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            (a, b) => Err(invalid_numeric_op(
                String::from("calculate remainder of"),
                a,
                b,
                location,
            )),
        }
    }

    pub fn eq(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a == b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn neq(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a != b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn lt(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a < b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn gt(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a > b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn lte(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a <= b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn gte(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Bool(a >= b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn and(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }

    pub fn or(&self, other: &Self, location: ast::Location) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            (a, b) => Err(invalid_comparison(a, b, location)),
        }
    }
}

fn invalid_numeric_op(op: String, a: &Value, b: &Value, location: ast::Location) -> RuntimeError {
    RuntimeError {
        message: format!("Cannot {:?} {:?} and {:?}", op, a, b),
        location,
        kind: RuntimeErrorKind::InvalidBinaryOperation,
    }
}

fn divison_by_zero(message: String, location: ast::Location) -> RuntimeError {
    RuntimeError {
        message,
        location,
        kind: RuntimeErrorKind::DivisionByZero,
    }
}

fn invalid_comparison(a: &Value, b: &Value, location: ast::Location) -> RuntimeError {
    RuntimeError {
        message: format!("Cannot compare {:?} and {:?}", a, b),
        location,
        kind: RuntimeErrorKind::InvalidBinaryOperation,
    }
}
