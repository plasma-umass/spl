use serde_json::Number;
extern crate serde_json;

#[derive(Debug, PartialEq)]
pub enum PatAtom {
  Select(String),
  Index(usize),
  Map(Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub enum Pat {
  Empty,
  Pat(PatAtom, Box<Pat>),
}

#[derive(Debug, PartialEq)]
pub enum Op {
  Eq,
  NotEq,
  Greater,
  Less,
  GreaterEq,
  LessEq
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Pat(Pat),
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Expr>),
    Object(Vec<(String,Expr)>),
    BinOp(Op, Box<Expr>, Box<Expr>)
}
