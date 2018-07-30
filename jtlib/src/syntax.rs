use serde_json::{Number, Value};
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

fn eval_pat(pat: &Pat, value: &Value) -> Option<Value> {
    match pat {
        Pat::Empty => Option::Some(value.clone()),
        Pat::Dot(key, p) => value.as_object()
            .and_then(|map| map.get(key))
            .and_then(|v| eval_pat(p, v))
    }
}

/*
   template<T> class Box {
       T* ptr;

       Box(T *ptr) {
           this.ptr = ptr;
        }

        ~Box() {
            delete ptr;
        }

  }
*/

pub fn eval(expr: &Expr, value: &Value) -> Option<Value> {
    match expr {
        Expr::Pat(p) => eval_pat(&p, value),
        Expr::Null => Option::Some(Value::Null),
        Expr::Bool(b) => Option::Some(Value::Bool(*b)),
        Expr::Number(n) => Option::Some(Value::Number(n.clone())),
        Expr::String(s) => Option::Some(Value::String(s.clone())),
        Expr::Array(vec) => vec.iter()
            .map(|e| eval(e, value))
            .collect::<Option<Vec<Value>>>()
            .map(|v| Value::Array(v)),
        Expr::Object(m) => m.iter()
            .map(|kv| {
                let k = kv.0.clone();
                eval(&kv.1, value).map(|v| (k, v))
            })
            .collect::<Option<serde_json::Map<String,Value>>>()
            .map(|m| Value::Object(m))
    }
}