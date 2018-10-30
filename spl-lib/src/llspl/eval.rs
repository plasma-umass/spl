use futures;
use futures::Future;
use super::syntax::{Payload,Expr};
use super::error::Error;
use json_transformers;
pub type EvalResult<'a> = Box<Future<Item = Payload, Error = Error> + Send + 'a>;

use serde_json as json;

fn eval_json(jt_expr: &json_transformers::Expr,
  input: Payload) -> Result<Payload, Error> {
  let in_json = input.to_json()?;
  let out_json = json_transformers::eval(jt_expr, &in_json)
    .ok_or(Error::JsonEval)?;
  Ok(Payload::Json(out_json))
}

fn extract_split(input: Payload) -> Result<(json::Value, json::Value), Error> {
  let in_json = input.to_json()?;
  
  match in_json {
    json::Value::Object(mut m) => match (m.remove("x"), m.remove("y")) {
        (Some(x), Some(y)) => Ok((x, y)),
        _ => Err(Error::JsonEval)
    }
    _ => Err(Error::JsonEval)
  }
}

fn download<'a>(url: &str) -> EvalResult<'a> {
  use reqwest;

  let mut buf: Vec<u8> = vec![];
  reqwest::get(url).unwrap().copy_to(&mut buf).unwrap();
  let payload = Payload::from_vec(buf);
  Box::new(futures::future::result(Ok(payload)))
}

pub trait Eval : Sync {

  fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;

  fn fetch<'a,'b>(&'b self, path: &'b str) -> EvalResult<'a>;



  fn eval<'a>(&'a self, input: Payload, expr: &'a Expr) -> EvalResult<'a> {
    match expr {
      Expr::Pure(n) => self.invoke(n, input),
      Expr::Download(jt_expr) => eval_json(&jt_expr, input)
          .and_then(move |pl| pl.to_json())
          .map(move |url| download(url.as_str().unwrap()))
          .unwrap(),
      Expr::Seq(e1, e2) => Box::new(self.eval(input, e1)
          .and_then(move |result| self.eval(result, e2))),
      Expr::Fetch(path) => self.fetch(path),
      Expr::Project(jt_expr) =>
        Box::new(futures::future::result(eval_json(&jt_expr, input))),
      Expr::Split(e) =>
        Box::new(futures::future::result(extract_split(input))
          .and_then(move |(x,y)|
            self.eval(Payload::Json(x), e)
              .and_then(move |z| futures::future::result(z.to_json()))
              .map(move |z| Payload::Json(json!({ "x": z, "y": y }))))),
      Expr::If(e1, e2, e3) =>
        Box::new(self.eval(input.clone(), e1)
          .and_then(|r| futures::future::result(r.to_json()))
            .and_then(move |test_value|
              match test_value {
                json::Value::Bool(true) => self.eval(input, e2),
                json::Value::Bool(false) => self.eval(input, e3),
                _ => Box::new(futures::failed(Error::JsonEval))
              })),
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  struct MockEval { }

  impl Eval for MockEval {

    fn fetch<'a,'b>(&'b self, _path: &'b str) -> EvalResult<'a> {
        Box::new(futures::future::result(Ok(Payload::Json(json!({})))))
    }

    fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a> {
      let result = match name {
        "f" => input.to_json().map(|json|
          Payload::Json(json!({ "input": json, "receiver": "f" }))),
        _ => Result::Err(Error::InvokeError("unknown function".to_string()))
      };
      Box::new(futures::future::result(result))
    }
  }

  fn parse(code: &'static str) -> Expr {
    super::super::parser::parse(code).unwrap().1
  }

  #[test]
  fn test_project() {
    let exp = parse("project { \"x\": 10 }");
    let result = (MockEval{}).eval(Payload::Json(json!({ "y": 20 })), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "x": 10.0
    })));
  }

  #[test]
  fn test_seq() {
    let exp = parse("pure \"f\"; pure \"f\"");
    let result = (MockEval{}).eval(Payload::Json(json!({ "x": 10 })), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "input": {
          "input": { "x": 10 },
          "receiver": "f"
        },
        "receiver": "f"
    })));
  }

  #[test]
  fn test_split() {
    let exp = parse("split (pure \"f\")");
    let result = (MockEval{}).eval(Payload::Json(json!({ "x": 10, "y": 20 })), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "x": {
          "input": 10,
          "receiver": "f"
        },
        "y": 20
      })));
  }

  #[test]
  fn test_if_true() {
    let exp = parse("if (project $in) { project { \"value\": 1 } } else { project { \"value\": 2 } }");
    let result = (MockEval{}).eval(Payload::Json(json!(true)), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "value": 1.0
      })));
  }

  #[test]
  fn test_if_false() {
    let exp = parse("if (project $in) { project { \"value\": 1 } } else { project { \"value\": 2 } }");
    let result = (MockEval{}).eval(Payload::Json(json!(false)), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "value": 2.0
      })));
  }

 #[test]
  fn test_seq_proj() {
    let exp = parse("pure \"f\" ; project $in.receiver ; pure \"f\"");
    let result = (MockEval{}).eval(Payload::Json(json!({ "x": 10 })), &exp);
    assert!(result.wait().unwrap() ==
      Payload::Json(json!({
        "input": "f",
        "receiver": "f"
    })));
  }

 #[test]
  fn test_map_proj() {
    let exp = parse("project $in.map({ \"y\": $in.x })");
    let input = json!([ { "x": 10 }, { "x": 20 }]);
    let output = json!([ { "y": 10 }, { "y": 20 }]);
    let result = (MockEval{}).eval(Payload::Json(input), &exp);
    assert!(result.wait().unwrap() == Payload::Json(output));
  }

  #[test]
  fn test_proj_double_select() {
    let exp = parse("project $in.x.y");
    let input = json!({ "x": { "y": 10 } });
    let output = json!(10);
    let result = (MockEval{}).eval(Payload::Json(input), &exp);
    assert!(result.wait().unwrap() == Payload::Json(output));
  }
}