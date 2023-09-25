use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::ast;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    Tuple((Box<Value>, Box<Value>)),
    Closure(Closure),
}

#[derive(Debug, Clone)]
pub struct Closure {
    function: ast::Function,
    context: Context,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(int) => write!(f, "{int}"),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::Str(str) => write!(f, "{str}"),
            Value::Tuple((first, second)) => write!(f, "({first}, {second})"),
            Value::Closure { .. } => write!(f, "<#closure>"),
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
    let context = Context::new();
    if let Err(error) = evaluate(program.expression, &context) {
        println!("{}", error);
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    parent: Option<Rc<Context>>,
    current: Rc<RefCell<HashMap<String, Value>>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            parent: None,
            current: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, var: &str) -> Option<Value> {
        if let Some(val) = self.current.borrow().get(var).cloned() {
            Some(val)
        } else {
            self.parent.as_ref()?.get(var)
        }
    }

    pub fn set(&self, var: impl Into<String>, value: Value) {
        self.current.borrow_mut().insert(var.into(), value);
    }
}

fn evaluate(term: ast::Term, context: &Context) -> Result<Value, RuntimeError> {
    match term {
        ast::Term::Int(int) => Ok(Value::Int(int.value)),
        ast::Term::Str(str) => Ok(Value::Str(str.value)),
        ast::Term::Call(call) => evaluate_call(*call, context),
        ast::Term::Binary(binary) => evaluate_binary(*binary, context),
        ast::Term::Function(function) => Ok(Value::Closure(Closure {
            function: *function,
            context: context.clone(),
        })),
        ast::Term::Let(let_) => evaluate_let(*let_, context),
        ast::Term::If(if_) => evaluate_if(*if_, context),
        ast::Term::Print(print) => {
            let value = evaluate(print.value, context)?;
            println!("{}", value);
            Ok(value)
        }
        ast::Term::First(first) => match evaluate(first.value, context)? {
            Value::Tuple((value, _)) => Ok(*value),
            _ => Err(RuntimeError {
                message: String::from("Not a tuple"),
                location: first.location,
                kind: RuntimeErrorKind::ArgumentError,
            }),
        },
        ast::Term::Second(second) => match evaluate(second.value, context)? {
            Value::Tuple((_, value)) => Ok(*value),
            _ => Err(RuntimeError {
                message: String::from("Not a tuple"),
                location: second.location,
                kind: RuntimeErrorKind::ArgumentError,
            }),
        },
        ast::Term::Bool(bool) => Ok(Value::Bool(bool.value)),
        ast::Term::Tuple(tuple) => Ok(Value::Tuple((
            Box::new(evaluate(tuple.first, context)?),
            Box::new(evaluate(tuple.second, context)?),
        ))),
        ast::Term::Var(var) => evaluate_var(var, context),
    }
}

fn evaluate_call(call: ast::Call, context: &Context) -> Result<Value, RuntimeError> {
    match evaluate(call.callee, context)? {
        Value::Closure(closure) => {
            if call.arguments.len() != closure.function.parameters.len() {
                return Err(RuntimeError {
                    message: format!(
                        "Expected {} arguments, got {}",
                        closure.function.parameters.len(),
                        call.arguments.len()
                    ),
                    location: call.location,
                    kind: RuntimeErrorKind::ArgumentError,
                });
            }

            for (parameter, argument) in closure.function.parameters.into_iter().zip(call.arguments)
            {
                closure
                    .context
                    .set(parameter.text, evaluate(argument, context)?);
            }

            evaluate(closure.function.value, &context)
        }
        _ => Err(RuntimeError {
            message: String::from("Not a function"),
            location: call.location,
            kind: RuntimeErrorKind::ArgumentError,
        }),
    }
}

fn evaluate_binary(binary: ast::Binary, context: &Context) -> Result<Value, RuntimeError> {
    let lhs = evaluate(binary.lhs, context)?;
    let rhs = evaluate(binary.rhs, context)?;

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

fn evaluate_let(let_: ast::Let, context: &Context) -> Result<Value, RuntimeError> {
    let name = let_.name.text;
    let value = evaluate(let_.value, context)?;
    context.set(name, value);
    evaluate(let_.next, context)
}

fn evaluate_if(if_: ast::If, context: &Context) -> Result<Value, RuntimeError> {
    match evaluate(if_.condition, context)? {
        Value::Bool(true) => evaluate(if_.then, context),
        Value::Bool(false) => evaluate(if_.otherwise, context),
        _ => Err(RuntimeError {
            message: String::from("Invalid condition"),
            location: if_.location,
            kind: RuntimeErrorKind::ArgumentError,
        }),
    }
}

fn evaluate_var(var: ast::Var, context: &Context) -> Result<Value, RuntimeError> {
    context
        .get(&var.text)
        .ok_or_else(|| RuntimeError {
            message: format!("Undefined variable {}", var.text),
            location: var.location,
            kind: RuntimeErrorKind::ArgumentError,
        })
        .into()
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
