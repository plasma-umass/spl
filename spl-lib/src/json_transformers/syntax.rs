use serde_json::Number;
extern crate serde_json;

#[derive(Debug, PartialEq)]
pub enum Pat {
  Empty,
  Dot(String, Box<Pat>)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Pat(Pat),
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Expr>),
    Object(Vec<(String,Expr)>)
}
