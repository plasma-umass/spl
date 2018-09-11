pub mod parser;
mod syntax;
mod eval;

use serde_json::Value;

pub type Expr = syntax::Expr;


pub fn eval(expr: &Expr, value: &Value) -> Option<Value> {
  eval::eval(expr, value)
}
