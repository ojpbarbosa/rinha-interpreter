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

#[derive(Deserialize, Debug)]
pub struct Str {
    pub value: String,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Print {
    pub value: Box<Term>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Term {
    Str(Str),
    Print(Print),
}
