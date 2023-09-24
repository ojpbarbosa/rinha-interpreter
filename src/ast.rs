use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub start: i32,
    pub end: i32,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Box<Term>,
}

#[derive(Deserialize, Debug)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
}

#[derive(Deserialize, Debug)]
pub struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
    location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Deserialize, Debug)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Call(Call),
    Binary(Binary),
    Function(Function),
    Let(Let),
    If(If),
    Print(Print),
    First(First),
    Second(Second),
    Bool(Bool),
    Tuple(Tuple),
    Var(Var),
}
