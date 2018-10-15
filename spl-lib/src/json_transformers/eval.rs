use serde_json::{Value, Map};
extern crate serde_json;
use super::syntax::*;

fn eval_pat(pat: &Pat, value: &Value) -> Option<Value> {
    match pat {
        Pat::Empty => Option::Some(value.clone()),
        Pat::Pat(PatAtom::Select(key), p) => value.as_object()
            .and_then(|map| map.get(key))
            .and_then(|v| eval_pat(p, v)),
        Pat::Pat(PatAtom::Index(idx), p) => value.as_array()
            .and_then(|arr| arr.get(*idx))
            .and_then(|v| eval_pat(p, v)),
        Pat::Pat(PatAtom::Map(f), p) => match value {
            Value::Array(vec) => vec.iter()
                .map(|e| eval(f, e))
                .collect::<Option<Vec<Value>>>()
                .map(|v| Value::Array(v))
                .and_then(|v| eval_pat(p, &v)),
            _ => None
        }
    }
}

fn eval_binop(op: &Op, l: &Expr, r: &Expr, value: &Value) -> Option<Value> {
    let lv = eval(l, value);
    let rv = eval(r, value);
    match op {
        Op::Eq => Some(Value::Bool(lv == rv)),
        Op::NotEq => Some(Value::Bool(lv != rv)),
        Op::Greater => match (lv, rv) {
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_u64() && rn.is_u64() =>
                Some(Value::Bool(ln.as_u64() > rn.as_u64())),
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_f64() && rn.is_f64() =>
                Some(Value::Bool(ln.as_f64() > rn.as_f64())),
            _ => None
        }
        Op::Less => match (lv, rv) {
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_u64() && rn.is_u64() =>
                Some(Value::Bool(ln.as_u64() < rn.as_u64())),
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_f64() && rn.is_f64() =>
                Some(Value::Bool(ln.as_f64() < rn.as_f64())),
            _ => None
        }
        Op::GreaterEq => match (lv, rv) {
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_u64() && rn.is_u64() =>
                Some(Value::Bool(ln.as_u64() >= rn.as_u64())),
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_f64() && rn.is_f64() =>
                Some(Value::Bool(ln.as_f64() >= rn.as_f64())),
            _ => None
        }
        Op::LessEq => match (lv, rv) {
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_u64() && rn.is_u64() =>
                Some(Value::Bool(ln.as_u64() <= rn.as_u64())),
            (Some(Value::Number(ref ln)), Some(Value::Number(ref rn)))
            if ln.is_f64() && rn.is_f64() =>
                Some(Value::Bool(ln.as_f64() <= rn.as_f64())),
            _ => None
        }
    }
}

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
            .map(|m| Value::Object(m)),
        Expr::BinOp(op, l, r) => eval_binop(op, &*l, &*r, value)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use json_transformers::parser::{parse_string};

  #[test]
  fn test_binop_eq_true() {
    let exp = parse_string("$in.x == $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": 10 }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_eq_false() {
    let exp = parse_string("$in.x == $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": 1 }));
    assert!(result.unwrap() == Value::Bool(false));
  }

  #[test]
  fn test_binop_eq_strings() {
    let exp = parse_string("$in.x == $in.y");
    let result = eval(&exp, &json!({ "x": "abcde", "y": "abcde" }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_eq_objects() {
    let exp = parse_string("$in.x == $in.y");
    let result = eval(&exp, &json!({ "x": { "i": 100 }, "y": { "i": 100 } }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_neq_true() {
    let exp = parse_string("$in.x != $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": 1 }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_neq_false() {
    let exp = parse_string("$in.x != $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": 10 }));
    assert!(result.unwrap() == Value::Bool(false));
  }

  #[test]
  fn test_binop_neq_different_types() {
    let exp = parse_string("$in.x != $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": "10" }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_gt_true() {
    let exp = parse_string("$in.x > $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": 5 }));
    assert!(result.unwrap() == Value::Bool(true));
  }

  #[test]
  fn test_binop_gt_false() {
    let exp = parse_string("$in.x > $in.y");
    let result = eval(&exp, &json!({ "x": 1, "y": 1 }));
    assert!(result.unwrap() == Value::Bool(false));
  }

  #[test]
  fn test_binop_gt_different_types() {
    let exp = parse_string("$in.x > $in.y");
    let result = eval(&exp, &json!({ "x": 10, "y": "1" }));
    assert!(result.is_none());
  }
}