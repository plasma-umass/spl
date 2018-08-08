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

pub trait Eval : Sync {

  fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a>;

  fn fetch<'a,'b>(&'b self, path: &'b str) -> EvalResult<'a>;

  fn eval<'a,'b>(&'b self, input: Payload, expr: &'a Expr) -> EvalResult<'b>
  where 'a : 'b {
    match expr {
      Expr::Pure(n) => self.invoke(n, input),
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
        Box::new(futures::future::result(extract_split(input))
          .and_then(move |(x,y)|
            self.eval(Payload::Json(x), e1)
              .and_then(|r| futures::future::result(r.to_json()))
               .and_then(move |test_value|
                 match test_value {
                   json::Value::Bool(true) => self.eval(Payload::Json(y), e2),
                   json::Value::Bool(false) => self.eval(Payload::Json(y), e3),
                   _ => Box::new(futures::failed(Error::JsonEval))
                 })))
    }
  }

}