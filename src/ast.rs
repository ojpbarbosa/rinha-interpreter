use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Location {
    pub start: i32,
    pub end: i32,
    pub filename: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Call {
    pub callee: Term,
    pub arguments: Vec<Term>,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Let {
    pub name: Parameter,
    pub value: Term,
    pub next: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct If {
    pub condition: Term,
    pub then: Term,
    pub otherwise: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Binary {
    pub lhs: Term,
    pub op: BinaryOp,
    pub rhs: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tuple {
    pub first: Term,
    pub second: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct First {
    pub value: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Second {
    pub value: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Print {
    pub value: Term,
    pub location: Location,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Call(Box<Call>),
    Binary(Box<Binary>),
    Function(Box<Function>),
    Let(Box<Let>),
    If(Box<If>),
    Print(Box<Print>),
    First(Box<First>),
    Second(Box<Second>),
    Bool(Bool),
    Tuple(Box<Tuple>),
    Var(Var),
}
